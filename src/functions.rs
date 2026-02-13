//! Functions for CLR hosting.

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::core::{Error, GUID, HRESULT, Interface, s, w};

/// Function pointer type for CLRCreateInstance
type CLRCreateInstanceFn = unsafe extern "system" fn(
    clsid: *const GUID,
    riid: *const GUID,
    ppInterface: *mut *mut core::ffi::c_void,
) -> HRESULT;

/// Creates an instance of a CLR hosting interface.
///
/// This is the main entry point for CLR hosting on .NET 4.0 and later.
/// Use with `CLSID_CLRMetaHost` to get an `ICLRMetaHost` interface.
///
/// # Arguments
///
/// * `clsid` - The CLSID of the interface to create (e.g., `CLSID_CLRMetaHost`)
///
/// # Type Parameters
///
/// * `T` - The interface type to return (e.g., `ICLRMetaHost`)
///
/// # Example
///
/// ```no_run
/// use mscoree::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost};
///
/// unsafe {
///     let meta_host: ICLRMetaHost = CLRCreateInstance(&CLSID_CLRMetaHost)?;
/// }
/// # Ok::<(), windows::core::Error>(())
/// ```
///
/// # Safety
///
/// This function loads mscoree.dll and calls into COM interfaces.
pub unsafe fn CLRCreateInstance<T: Interface>(clsid: &GUID) -> Result<T, Error> {
    unsafe {
        // Load mscoree.dll
        let module_name = w!("mscoree.dll");
        let module: HMODULE = LoadLibraryW(module_name)?;

        // Get CLRCreateInstance function pointer
        let proc_name = s!("CLRCreateInstance");
        let proc_addr = GetProcAddress(module, proc_name).ok_or_else(|| Error::from_win32())?;

        let clr_create_instance: CLRCreateInstanceFn = core::mem::transmute(proc_addr);

        // Call CLRCreateInstance
        let mut result: *mut core::ffi::c_void = core::ptr::null_mut();
        let hr = clr_create_instance(clsid, &T::IID, &mut result);

        if hr.is_ok() {
            Ok(T::from_raw(result))
        } else {
            Err(Error::from(hr))
        }
    }
}
