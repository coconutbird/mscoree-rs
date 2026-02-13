//! Example demonstrating how to enumerate .NET assemblies in a process using DAC.
//!
//! This example shows how to:
//! 1. Open a target .NET process
//! 2. Load the DAC (Data Access Component) DLL
//! 3. Create an IXCLRDataProcess interface
//! 4. Enumerate AppDomains, Assemblies, and Modules
//!
//! # Prerequisites
//! - A running .NET process (Framework or Core)
//! - The appropriate DAC DLL (mscordacwks.dll or mscordaccore.dll)
//!
//! # Usage
//! ```
//! cargo run --example enumerate_assemblies -- <PID>
//! ```
//!
//! # Note
//! This example implements ICLRDataTarget using a manual vtable-based approach
//! to read memory from the target process.

use std::collections::HashMap;
use std::env;
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use windows::core::{GUID, HRESULT, IUnknown, Interface};
use windows::Win32::Foundation::{HANDLE, E_NOTIMPL, S_OK, E_FAIL, CloseHandle, MAX_PATH};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::ProcessStatus::{EnumProcessModulesEx, GetModuleBaseNameW, GetModuleInformation, LIST_MODULES_ALL, MODULEINFO};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_VM_READ, PROCESS_VM_WRITE, PROCESS_QUERY_INFORMATION};
use windows::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};

use mscoree::{
    IXCLRDataProcess, IXCLRDataAppDomain, IXCLRDataAssembly, IXCLRDataModule,
    CLRDATA_ENUM, IID_IXCLRDataProcess, IID_ICLRDataTarget,
};

/// Function pointer type for CLRDataCreateInstance from the DAC DLL
type CLRDataCreateInstanceFn = unsafe extern "system" fn(
    iid: *const GUID,
    target: *mut c_void,
    iface: *mut *mut c_void,
) -> HRESULT;

// Manual COM implementation of ICLRDataTarget
// We need to implement this manually because the #[implement] macro
// doesn't work well with our custom #[interface] definitions.

/// VTable for ICLRDataTarget - matches the COM interface layout
#[repr(C)]
struct ICLRDataTargetVtbl {
    // IUnknown methods
    query_interface: unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "system" fn(*mut c_void) -> u32,
    release: unsafe extern "system" fn(*mut c_void) -> u32,
    // ICLRDataTarget methods
    get_machine_type: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    get_pointer_size: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    get_image_base: unsafe extern "system" fn(*mut c_void, *const u16, *mut u64) -> HRESULT,
    read_virtual: unsafe extern "system" fn(*mut c_void, u64, *mut u8, u32, *mut u32) -> HRESULT,
    write_virtual: unsafe extern "system" fn(*mut c_void, u64, *const u8, u32, *mut u32) -> HRESULT,
    get_tls_value: unsafe extern "system" fn(*mut c_void, u32, u32, *mut u64) -> HRESULT,
    set_tls_value: unsafe extern "system" fn(*mut c_void, u32, u32, u64) -> HRESULT,
    get_current_thread_id: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    get_thread_context: unsafe extern "system" fn(*mut c_void, u32, u32, u32, *mut u8) -> HRESULT,
    set_thread_context: unsafe extern "system" fn(*mut c_void, u32, u32, *const u8) -> HRESULT,
    request: unsafe extern "system" fn(*mut c_void, u32, u32, *const u8, u32, *mut u8) -> HRESULT,
}

/// Our implementation of ICLRDataTarget for live process memory reading
#[allow(dead_code)]
struct LiveProcessDataTarget {
    /// Pointer to vtable - must be first field for COM compatibility
    vtbl: *const ICLRDataTargetVtbl,
    ref_count: AtomicU32,
    process_handle: HANDLE,
    pointer_size: u32,
    /// Cache of module name -> base address (populated on first GetImageBase call)
    module_cache: RwLock<HashMap<String, u64>>,
}

// Static vtable instance
static LIVE_PROCESS_DATA_TARGET_VTBL: ICLRDataTargetVtbl = ICLRDataTargetVtbl {
    query_interface: LiveProcessDataTarget::query_interface,
    add_ref: LiveProcessDataTarget::add_ref,
    release: LiveProcessDataTarget::release,
    get_machine_type: LiveProcessDataTarget::get_machine_type,
    get_pointer_size: LiveProcessDataTarget::get_pointer_size,
    get_image_base: LiveProcessDataTarget::get_image_base,
    read_virtual: LiveProcessDataTarget::read_virtual,
    write_virtual: LiveProcessDataTarget::write_virtual,
    get_tls_value: LiveProcessDataTarget::get_tls_value,
    set_tls_value: LiveProcessDataTarget::set_tls_value,
    get_current_thread_id: LiveProcessDataTarget::get_current_thread_id,
    get_thread_context: LiveProcessDataTarget::get_thread_context,
    set_thread_context: LiveProcessDataTarget::set_thread_context,
    request: LiveProcessDataTarget::request,
};

impl LiveProcessDataTarget {
    fn new(process_handle: HANDLE) -> Box<Self> {
        Box::new(Self {
            vtbl: &LIVE_PROCESS_DATA_TARGET_VTBL,
            ref_count: AtomicU32::new(1),
            process_handle,
            #[cfg(target_arch = "x86_64")]
            pointer_size: 8,
            #[cfg(target_arch = "x86")]
            pointer_size: 4,
            module_cache: RwLock::new(HashMap::new()),
        })
    }

    /// Populate the module cache by enumerating all modules in the process
    fn populate_module_cache(&self) {
        let mut cache = self.module_cache.write().unwrap();
        if !cache.is_empty() {
            return; // Already populated
        }

        unsafe {
            let mut modules = [std::mem::zeroed::<windows::Win32::Foundation::HMODULE>(); 1024];
            let mut cb_needed: u32 = 0;

            if EnumProcessModulesEx(
                self.process_handle,
                modules.as_mut_ptr(),
                (modules.len() * std::mem::size_of::<windows::Win32::Foundation::HMODULE>()) as u32,
                &mut cb_needed,
                LIST_MODULES_ALL,
            ).is_ok() {
                let module_count = cb_needed as usize / std::mem::size_of::<windows::Win32::Foundation::HMODULE>();

                for i in 0..module_count {
                    let module = modules[i];
                    let mut name_buf = [0u16; MAX_PATH as usize];

                    let name_len = GetModuleBaseNameW(self.process_handle, Some(module), &mut name_buf);
                    if name_len > 0 {
                        let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                        let name_lower = name.to_lowercase();

                        let mut mod_info: MODULEINFO = std::mem::zeroed();
                        if GetModuleInformation(
                            self.process_handle,
                            module,
                            &mut mod_info,
                            std::mem::size_of::<MODULEINFO>() as u32,
                        ).is_ok() {
                            let base_addr = mod_info.lpBaseOfDll as u64;
                            cache.insert(name_lower, base_addr);
                        }
                    }
                }
            }
        }
    }

    unsafe extern "system" fn query_interface(
        this: *mut c_void,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        let iid = unsafe { &*riid };

        // Support IUnknown and ICLRDataTarget
        if *iid == IUnknown::IID || *iid == IID_ICLRDataTarget {
            unsafe {
                *ppv = this;
                Self::add_ref(this);
            }
            S_OK
        } else {
            unsafe { *ppv = ptr::null_mut(); }
            HRESULT(-2147467262i32) // E_NOINTERFACE
        }
    }

    unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
        let target = unsafe { &*(this as *const Self) };
        target.ref_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        let target = unsafe { &*(this as *const Self) };
        let count = target.ref_count.fetch_sub(1, Ordering::SeqCst) - 1;
        if count == 0 {
            // Don't actually drop - we manage lifetime manually
        }
        count
    }

    unsafe extern "system" fn get_machine_type(_this: *mut c_void, machine_type: *mut u32) -> HRESULT {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            { *machine_type = 0x8664; } // IMAGE_FILE_MACHINE_AMD64
            #[cfg(target_arch = "x86")]
            { *machine_type = 0x014c; } // IMAGE_FILE_MACHINE_I386
        }
        S_OK
    }

    unsafe extern "system" fn get_pointer_size(this: *mut c_void, pointer_size: *mut u32) -> HRESULT {
        let target = unsafe { &*(this as *const Self) };
        unsafe { *pointer_size = target.pointer_size; }
        S_OK
    }

    unsafe extern "system" fn get_image_base(
        this: *mut c_void,
        image_path: *const u16,
        base_address: *mut u64,
    ) -> HRESULT {
        let target = unsafe { &*(this as *const Self) };

        // Convert the image path to a string
        let path_str = unsafe {
            let mut len = 0;
            while *image_path.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(image_path, len);
            String::from_utf16_lossy(slice)
        };

        // Extract just the filename from the path
        let filename = path_str
            .rsplit(|c| c == '\\' || c == '/')
            .next()
            .unwrap_or(&path_str)
            .to_lowercase();

        // Populate the module cache if needed
        target.populate_module_cache();

        // Look up the module in the cache
        let cache = target.module_cache.read().unwrap();
        if let Some(&addr) = cache.get(&filename) {
            unsafe { *base_address = addr; }
            S_OK
        } else {
            // Try partial match (e.g., "clr.dll" might be requested as "clr")
            for (name, &addr) in cache.iter() {
                if name.starts_with(&filename) || filename.starts_with(name.trim_end_matches(".dll")) {
                    unsafe { *base_address = addr; }
                    return S_OK;
                }
            }
            unsafe { *base_address = 0; }
            E_FAIL
        }
    }

    unsafe extern "system" fn read_virtual(
        this: *mut c_void,
        address: u64,
        buffer: *mut u8,
        bytes_requested: u32,
        bytes_done: *mut u32,
    ) -> HRESULT {
        let target = unsafe { &*(this as *const Self) };
        let mut bytes_read: usize = 0;
        let result = unsafe {
            ReadProcessMemory(
                target.process_handle,
                address as *const c_void,
                buffer as *mut c_void,
                bytes_requested as usize,
                Some(&mut bytes_read),
            )
        };

        unsafe {
            if !bytes_done.is_null() {
                *bytes_done = bytes_read as u32;
            }
        }

        if result.is_ok() { S_OK } else { E_FAIL }
    }

    unsafe extern "system" fn write_virtual(
        this: *mut c_void,
        address: u64,
        buffer: *const u8,
        bytes_requested: u32,
        bytes_done: *mut u32,
    ) -> HRESULT {
        let target = unsafe { &*(this as *const Self) };
        let mut bytes_written: usize = 0;
        let result = unsafe {
            WriteProcessMemory(
                target.process_handle,
                address as *const c_void,
                buffer as *const c_void,
                bytes_requested as usize,
                Some(&mut bytes_written),
            )
        };

        unsafe {
            if !bytes_done.is_null() {
                *bytes_done = bytes_written as u32;
            }
        }

        if result.is_ok() { S_OK } else { E_FAIL }
    }

    unsafe extern "system" fn get_tls_value(
        _this: *mut c_void, _thread_id: u32, _index: u32, _value: *mut u64,
    ) -> HRESULT {
        E_NOTIMPL
    }

    unsafe extern "system" fn set_tls_value(
        _this: *mut c_void, _thread_id: u32, _index: u32, _value: u64,
    ) -> HRESULT {
        E_NOTIMPL
    }

    unsafe extern "system" fn get_current_thread_id(_this: *mut c_void, _thread_id: *mut u32) -> HRESULT {
        E_NOTIMPL
    }

    unsafe extern "system" fn get_thread_context(
        _this: *mut c_void, _thread_id: u32, _context_flags: u32, _context_size: u32, _context: *mut u8,
    ) -> HRESULT {
        E_NOTIMPL
    }

    unsafe extern "system" fn set_thread_context(
        _this: *mut c_void, _thread_id: u32, _context_size: u32, _context: *const u8,
    ) -> HRESULT {
        E_NOTIMPL
    }

    unsafe extern "system" fn request(
        _this: *mut c_void, _req_code: u32, _in_size: u32, _in_buf: *const u8, _out_size: u32, _out_buf: *mut u8,
    ) -> HRESULT {
        E_NOTIMPL
    }
}

/// Helper to convert a wide string buffer to a Rust String
fn wide_to_string(buffer: &[u16], len: u32) -> String {
    let slice = &buffer[..len as usize];
    // Find null terminator if present
    let end = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
    String::from_utf16_lossy(&slice[..end])
}

/// Find the CLR module path in the target process and return the DAC DLL path
fn find_dac_path(process_handle: HANDLE) -> Option<String> {
    use windows::Win32::System::ProcessStatus::{EnumProcessModulesEx, GetModuleFileNameExW, LIST_MODULES_ALL};

    unsafe {
        let mut modules = [std::mem::zeroed::<windows::Win32::Foundation::HMODULE>(); 1024];
        let mut cb_needed: u32 = 0;

        if EnumProcessModulesEx(
            process_handle,
            modules.as_mut_ptr(),
            (modules.len() * std::mem::size_of::<windows::Win32::Foundation::HMODULE>()) as u32,
            &mut cb_needed,
            LIST_MODULES_ALL,
        ).is_ok() {
            let module_count = cb_needed as usize / std::mem::size_of::<windows::Win32::Foundation::HMODULE>();

            for i in 0..module_count {
                let module = modules[i];
                let mut path_buf = [0u16; 1024];

                let path_len = GetModuleFileNameExW(Some(process_handle), Some(module), &mut path_buf);
                if path_len > 0 {
                    let path = String::from_utf16_lossy(&path_buf[..path_len as usize]);
                    let path_lower = path.to_lowercase();

                    // Look for CLR modules
                    if path_lower.ends_with("coreclr.dll") {
                        // .NET Core/.NET 5+ - DAC is mscordaccore.dll in same directory
                        if let Some(dir) = std::path::Path::new(&path).parent() {
                            let dac_path = dir.join("mscordaccore.dll");
                            if dac_path.exists() {
                                return Some(dac_path.to_string_lossy().to_string());
                            }
                        }
                    } else if path_lower.ends_with("clr.dll") {
                        // .NET Framework - DAC is mscordacwks.dll in same directory
                        if let Some(dir) = std::path::Path::new(&path).parent() {
                            let dac_path = dir.join("mscordacwks.dll");
                            if dac_path.exists() {
                                return Some(dac_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <PID> [DAC_PATH]", args[0]);
        eprintln!("  PID: Process ID of a running .NET process");
        eprintln!("  DAC_PATH: Optional path to mscordaccore.dll or mscordacwks.dll");
        std::process::exit(1);
    }

    let pid: u32 = args[1].parse().map_err(|_| "Invalid PID")?;
    let dac_path_arg = args.get(2).cloned();

    println!("Enumerating .NET assemblies in process {}", pid);
    println!("============================================\n");

    unsafe {
        // Open the target process
        let process_handle = OpenProcess(
            PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION,
            false,
            pid,
        )?;

        if process_handle.is_invalid() {
            return Err("Failed to open process".into());
        }

        // Create our data target implementation (manual COM implementation)
        let data_target = LiveProcessDataTarget::new(process_handle);
        let data_target_ptr = Box::into_raw(data_target) as *mut c_void;

        // Find and load the DAC DLL
        let dac_path = if let Some(path) = dac_path_arg {
            println!("Using provided DAC path: {}", path);
            path
        } else if let Some(path) = find_dac_path(process_handle) {
            println!("Found DAC at: {}", path);
            path
        } else {
            return Err("Could not find DAC DLL. Please provide the path as second argument.\n\
                       For .NET Core: path to mscordaccore.dll\n\
                       For .NET Framework: path to mscordacwks.dll".into());
        };

        // Convert to wide string and load
        let dac_path_wide: Vec<u16> = dac_path.encode_utf16().chain(std::iter::once(0)).collect();
        let dac_module = LoadLibraryW(windows::core::PCWSTR::from_raw(dac_path_wide.as_ptr()))?;
        println!("Loaded DAC DLL successfully");

        // Get CLRDataCreateInstance function
        let proc_name = windows::core::s!("CLRDataCreateInstance");
        let proc_addr = GetProcAddress(dac_module, proc_name)
            .ok_or("Failed to get CLRDataCreateInstance")?;

        let clr_data_create_instance: CLRDataCreateInstanceFn =
            std::mem::transmute(proc_addr);

        // Create IXCLRDataProcess
        let mut process_ptr: *mut c_void = ptr::null_mut();
        let hr = clr_data_create_instance(
            &IID_IXCLRDataProcess,
            data_target_ptr,
            &mut process_ptr,
        );

        if hr.is_err() {
            CloseHandle(process_handle)?;
            return Err(format!("CLRDataCreateInstance failed: {:?}", hr).into());
        }

        let xclr_process: IXCLRDataProcess = IXCLRDataProcess::from_raw(process_ptr);
        println!("Successfully created IXCLRDataProcess\n");

        // Enumerate AppDomains
        println!("AppDomains:");
        println!("-----------");
        let mut app_domain_enum: CLRDATA_ENUM = 0;
        if xclr_process.StartEnumAppDomains(&mut app_domain_enum).is_ok() {
            loop {
                let mut app_domain_ptr: *mut IUnknown = ptr::null_mut();
                let hr = xclr_process.EnumAppDomain(&mut app_domain_enum, &mut app_domain_ptr);

                if hr.is_err() || app_domain_ptr.is_null() {
                    break;
                }

                // Cast to IXCLRDataAppDomain
                if let Ok(app_domain) = (*app_domain_ptr).cast::<IXCLRDataAppDomain>() {
                    let mut name_buf = [0u16; 512];
                    let mut name_len: u32 = 0;

                    if app_domain.GetName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let name = wide_to_string(&name_buf, name_len);
                        println!("  AppDomain: {}", name);
                    }
                }
            }
            let _ = xclr_process.EndEnumAppDomains(app_domain_enum);
        }

        // Enumerate Assemblies
        println!("\nAssemblies:");
        println!("-----------");
        let mut assembly_enum: CLRDATA_ENUM = 0;
        if xclr_process.StartEnumAssemblies(&mut assembly_enum).is_ok() {
            loop {
                let mut assembly_ptr: *mut IUnknown = ptr::null_mut();
                let hr = xclr_process.EnumAssembly(&mut assembly_enum, &mut assembly_ptr);

                if hr.is_err() || assembly_ptr.is_null() {
                    break;
                }

                // Cast to IXCLRDataAssembly
                if let Ok(assembly) = (*assembly_ptr).cast::<IXCLRDataAssembly>() {
                    let mut name_buf = [0u16; 1024];
                    let mut name_len: u32 = 0;

                    // Get display name
                    if assembly.GetDisplayName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let name = wide_to_string(&name_buf, name_len);
                        println!("  Assembly: {}", name);
                    } else if assembly.GetName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let name = wide_to_string(&name_buf, name_len);
                        println!("  Assembly: {}", name);
                    }

                    // Get file name
                    if assembly.GetFileName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let filename = wide_to_string(&name_buf, name_len);
                        println!("    File: {}", filename);
                    }
                }
            }
            let _ = xclr_process.EndEnumAssemblies(assembly_enum);
        }

        // Enumerate Modules
        println!("\nModules:");
        println!("--------");
        let mut module_enum: CLRDATA_ENUM = 0;
        if xclr_process.StartEnumModules(&mut module_enum).is_ok() {
            loop {
                let mut module_ptr: *mut IUnknown = ptr::null_mut();
                let hr = xclr_process.EnumModule(&mut module_enum, &mut module_ptr);

                if hr.is_err() || module_ptr.is_null() {
                    break;
                }

                // Cast to IXCLRDataModule
                if let Ok(module) = (*module_ptr).cast::<IXCLRDataModule>() {
                    let mut name_buf = [0u16; 1024];
                    let mut name_len: u32 = 0;

                    if module.GetName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let name = wide_to_string(&name_buf, name_len);
                        println!("  Module: {}", name);
                    }

                    // Get file name
                    if module.GetFileName(
                        name_buf.len() as u32,
                        &mut name_len,
                        name_buf.as_mut_ptr(),
                    ).is_ok() {
                        let filename = wide_to_string(&name_buf, name_len);
                        println!("    File: {}", filename);
                    }
                }
            }
            let _ = xclr_process.EndEnumModules(module_enum);
        }

        // Cleanup
        // Free our data target (we own it)
        drop(Box::from_raw(data_target_ptr as *mut LiveProcessDataTarget));
        CloseHandle(process_handle)?;

        println!("\nEnumeration complete!");
    }

    Ok(())
}

