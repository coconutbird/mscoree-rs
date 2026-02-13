//! ICorJitCompiler interface definitions for .NET JIT compilation.
//!
//! These interfaces provide access to the JIT compiler and allow hooking
//! or replacing the JIT compilation process.
//!
//! Note: Unlike most COM interfaces in this crate, `ICorJitCompiler` is a pure C++
//! virtual class that does not inherit from `IUnknown`. It uses a simple vtable layout.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use windows::core::{GUID, w};

/// Result codes returned by ICorJitCompiler::compileMethod
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorJitResult {
    /// Compilation succeeded
    CORJIT_OK = 0,
    /// Bad IL code
    CORJIT_BADCODE = 0x80000001u32 as i32,
    /// Out of memory
    CORJIT_OUTOFMEM = 0x80000002u32 as i32,
    /// Internal error
    CORJIT_INTERNALERROR = 0x80000003u32 as i32,
    /// Compilation skipped
    CORJIT_SKIPPED = 0x80000004u32 as i32,
    /// Recoverable error
    CORJIT_RECOVERABLEERROR = 0x80000005u32 as i32,
    /// Implementation limitation
    CORJIT_IMPLLIMITATION = 0x80000006u32 as i32,
    /// ReadyToRun not supported
    CORJIT_R2R_UNSUPPORTED = 0x80000007u32 as i32,
}

/// Flags passed to ICorJitInfo::allocMem
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorJitAllocMemFlag {
    CORJIT_ALLOCMEM_HOT_CODE = 1,
    CORJIT_ALLOCMEM_COLD_CODE = 2,
    CORJIT_ALLOCMEM_READONLY_DATA = 4,
    CORJIT_ALLOCMEM_HAS_POINTERS_TO_CODE = 8,
}

/// Function kind for exception handling
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorJitFuncKind {
    /// The main/root function (always id==0)
    CORJIT_FUNC_ROOT = 0,
    /// A funclet associated with an EH handler (finally, fault, catch, filter handler)
    CORJIT_FUNC_HANDLER = 1,
    /// A funclet associated with an EH filter
    CORJIT_FUNC_FILTER = 2,
}

/// Opaque handle to method information provided by the EE
pub type CORINFO_METHOD_HANDLE = *mut c_void;

/// Opaque handle to module information provided by the EE
pub type CORINFO_MODULE_HANDLE = *mut c_void;

/// Opaque handle to class information provided by the EE
pub type CORINFO_CLASS_HANDLE = *mut c_void;

/// Opaque handle to field information provided by the EE
pub type CORINFO_FIELD_HANDLE = *mut c_void;

/// Opaque handle to the argument list
pub type CORINFO_ARG_LIST_HANDLE = *mut c_void;

/// Signature information structure (simplified - full structure is complex)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CORINFO_SIG_INFO {
    pub callConv: u32,
    pub retTypeClass: CORINFO_CLASS_HANDLE,
    pub retTypeSigClass: CORINFO_CLASS_HANDLE,
    pub retType: u8,
    pub flags: u8,
    pub numArgs: u16,
    pub sigInst_classInstCount: u32,
    pub sigInst_classInst: *mut CORINFO_CLASS_HANDLE,
    pub sigInst_methInstCount: u32,
    pub sigInst_methInst: *mut CORINFO_CLASS_HANDLE,
    pub args: CORINFO_ARG_LIST_HANDLE,
    pub pSig: *const u8,
    pub cbSig: u32,
    pub methodSignature: *mut c_void,
    pub scope: CORINFO_MODULE_HANDLE,
    pub token: u32,
}

/// Method information passed to compileMethod
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CORINFO_METHOD_INFO {
    /// Handle to the method being compiled
    pub ftn: CORINFO_METHOD_HANDLE,
    /// Handle to the module containing the method
    pub scope: CORINFO_MODULE_HANDLE,
    /// Pointer to the IL code
    pub ILCode: *const u8,
    /// Size of the IL code in bytes
    pub ILCodeSize: u32,
    /// Maximum stack depth
    pub maxStack: u32,
    /// Number of EH clauses
    pub EHcount: u32,
    /// Method options/flags
    pub options: u32,
    /// Region kind
    pub regionKind: u32,
    /// Method signature information
    pub args: CORINFO_SIG_INFO,
    /// Local variable signature information  
    pub locals: CORINFO_SIG_INFO,
}

/// Target OS for JIT compilation
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CORINFO_OS {
    CORINFO_WINNT = 0,
    CORINFO_UNIX = 1,
    CORINFO_APPLE = 2,
}

/// ICorJitInfo interface (opaque - passed to compileMethod)
/// This is the interface the JIT uses to call back into the EE.
pub type ICorJitInfo = c_void;

/// ICorStaticInfo interface (opaque - passed to ProcessShutdownWork)
pub type ICorStaticInfo = c_void;

/// ICorJitHost interface (opaque - passed to jitStartup)
/// This is the host interface that the JIT calls back into for memory allocation, etc.
pub type ICorJitHost = c_void;

// Use C calling convention for x64, thiscall for x86
// On x64 Windows, the first argument (this) is passed in RCX which matches the C convention
// On x86 Windows, thiscall passes 'this' in ECX

/// VTable for ICorJitCompiler
///
/// This is the virtual function table layout for the ICorJitCompiler C++ class.
/// The JIT compiler interface does not inherit from IUnknown - it's a pure C++ virtual class.
///
/// Note: On x64, the calling convention is effectively "C" since the first argument (this)
/// is passed in RCX. On x86, it uses thiscall where 'this' is in ECX.
#[repr(C)]
pub struct ICorJitCompilerVtbl {
    /// compileMethod - Main entry point to compile IL to native code
    ///
    /// # Arguments
    /// * `comp` - ICorJitInfo interface for callbacks to EE
    /// * `info` - Information about the method to compile
    /// * `flags` - JIT compilation flags (CorJitFlag)
    /// * `nativeEntry` - OUT: Pointer to the generated native code
    /// * `nativeSizeOfCode` - OUT: Size of the generated native code
    #[cfg(target_arch = "x86")]
    pub compileMethod: unsafe extern "thiscall" fn(
        this: *mut ICorJitCompiler,
        comp: *mut ICorJitInfo,
        info: *mut CORINFO_METHOD_INFO,
        flags: u32,
        nativeEntry: *mut *mut u8,
        nativeSizeOfCode: *mut u32,
    ) -> CorJitResult,

    #[cfg(not(target_arch = "x86"))]
    pub compileMethod: unsafe extern "C" fn(
        this: *mut ICorJitCompiler,
        comp: *mut ICorJitInfo,
        info: *mut CORINFO_METHOD_INFO,
        flags: u32,
        nativeEntry: *mut *mut u8,
        nativeSizeOfCode: *mut u32,
    ) -> CorJitResult,

    /// ProcessShutdownWork - Called at process shutdown
    #[cfg(target_arch = "x86")]
    pub ProcessShutdownWork: unsafe extern "thiscall" fn(
        this: *mut ICorJitCompiler,
        info: *mut ICorStaticInfo,
    ),

    #[cfg(not(target_arch = "x86"))]
    pub ProcessShutdownWork: unsafe extern "C" fn(
        this: *mut ICorJitCompiler,
        info: *mut ICorStaticInfo,
    ),

    /// getVersionIdentifier - Get the JIT/EE interface version
    ///
    /// The EE uses this to verify compatibility with the JIT.
    #[cfg(target_arch = "x86")]
    pub getVersionIdentifier: unsafe extern "thiscall" fn(
        this: *mut ICorJitCompiler,
        versionIdentifier: *mut GUID,
    ),

    #[cfg(not(target_arch = "x86"))]
    pub getVersionIdentifier: unsafe extern "C" fn(
        this: *mut ICorJitCompiler,
        versionIdentifier: *mut GUID,
    ),

    /// setTargetOS - Set the target OS for compilation
    ///
    /// Must be called before compileMethod is called for the first time.
    #[cfg(target_arch = "x86")]
    pub setTargetOS: unsafe extern "thiscall" fn(
        this: *mut ICorJitCompiler,
        os: CORINFO_OS,
    ),

    #[cfg(not(target_arch = "x86"))]
    pub setTargetOS: unsafe extern "C" fn(
        this: *mut ICorJitCompiler,
        os: CORINFO_OS,
    ),
}

/// ICorJitCompiler - The main JIT compiler interface
///
/// This interface is used by the Execution Engine (EE) to request
/// compilation of IL bytecode to native code.
///
/// # Note
///
/// Unlike COM interfaces, ICorJitCompiler is a pure C++ virtual class
/// that does not inherit from IUnknown. It uses a simple vtable layout.
///
/// # Example
///
/// ```no_run
/// use mscoree::interfaces::cor_jit::{ICorJitCompiler, getJit};
///
/// unsafe {
///     // Get the JIT compiler
///     let jit = getJit();
///     if !jit.is_null() {
///         // Get version identifier
///         let mut version = std::mem::zeroed();
///         (*(*jit).vtbl).getVersionIdentifier(jit, &mut version);
///     }
/// }
/// ```
#[repr(C)]
pub struct ICorJitCompiler {
    pub vtbl: *const ICorJitCompilerVtbl,
}

impl ICorJitCompiler {
    /// Compile a method from IL to native code.
    ///
    /// # Safety
    ///
    /// All pointers must be valid. The caller must ensure the JIT compiler
    /// is properly initialized and setTargetOS has been called.
    #[inline]
    pub unsafe fn compileMethod(
        &mut self,
        comp: *mut ICorJitInfo,
        info: *mut CORINFO_METHOD_INFO,
        flags: u32,
        nativeEntry: *mut *mut u8,
        nativeSizeOfCode: *mut u32,
    ) -> CorJitResult {
        unsafe {
            ((*self.vtbl).compileMethod)(self, comp, info, flags, nativeEntry, nativeSizeOfCode)
        }
    }

    /// Called at process shutdown.
    ///
    /// # Safety
    ///
    /// Must only be called during process shutdown.
    #[inline]
    pub unsafe fn ProcessShutdownWork(&mut self, info: *mut ICorStaticInfo) {
        unsafe {
            ((*self.vtbl).ProcessShutdownWork)(self, info)
        }
    }

    /// Get the JIT/EE interface version identifier.
    ///
    /// # Safety
    ///
    /// The versionIdentifier pointer must be valid.
    #[inline]
    pub unsafe fn getVersionIdentifier(&mut self, versionIdentifier: *mut GUID) {
        unsafe {
            ((*self.vtbl).getVersionIdentifier)(self, versionIdentifier)
        }
    }

    /// Set the target OS for compilation.
    ///
    /// Must be called before the first call to compileMethod.
    ///
    /// # Safety
    ///
    /// This must be called exactly once before any compilation.
    #[inline]
    pub unsafe fn setTargetOS(&mut self, os: CORINFO_OS) {
        unsafe {
            ((*self.vtbl).setTargetOS)(self, os)
        }
    }
}

// External functions from clrjit.dll
unsafe extern "C" {
    /// Get the JIT compiler instance.
    ///
    /// This function is exported from clrjit.dll and returns a pointer
    /// to the ICorJitCompiler interface.
    ///
    /// # Returns
    ///
    /// A pointer to the ICorJitCompiler interface, or null if not available.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it returns a raw pointer to a C++ object.
    /// The caller must ensure the JIT DLL is loaded before calling this.
    pub fn getJit() -> *mut ICorJitCompiler;

    /// Initialize the JIT with a host interface.
    ///
    /// This function must be called before getJit() to provide the JIT
    /// with a host interface for memory allocation and other services.
    ///
    /// # Arguments
    ///
    /// * `host` - Pointer to the ICorJitHost interface
    ///
    /// # Safety
    ///
    /// The host pointer must be valid and remain valid for the lifetime
    /// of the JIT compiler.
    pub fn jitStartup(host: *mut ICorJitHost);
}

/// Helper function to get the JIT compiler by loading the appropriate JIT DLL.
///
/// This function tries to load the JIT compiler in the following order:
/// 1. `clrjit.dll` - .NET Framework 4.0+, .NET Core, .NET 5+
/// 2. `mscorjit.dll` - .NET Framework 2.0, 3.0, 3.5 (CLR 2.0)
///
/// # Safety
///
/// This function loads a DLL and calls an exported function.
pub unsafe fn get_jit_compiler() -> Option<*mut ICorJitCompiler> {
    // Try clrjit.dll first (CLR 4.0+ and .NET Core)
    if let Some(jit) = unsafe { get_jit_from_dll(w!("clrjit.dll")) } {
        return Some(jit);
    }

    // Fall back to mscorjit.dll (CLR 2.0)
    unsafe { get_jit_from_dll(w!("mscorjit.dll")) }
}

/// Helper function to get the JIT compiler from clrjit.dll specifically.
///
/// Use this when you know you're targeting .NET Framework 4.0+ or .NET Core/.NET 5+.
///
/// # Safety
///
/// This function loads a DLL and calls an exported function.
pub unsafe fn get_jit_compiler_clr4() -> Option<*mut ICorJitCompiler> {
    unsafe { get_jit_from_dll(w!("clrjit.dll")) }
}

/// Helper function to get the JIT compiler from mscorjit.dll specifically.
///
/// Use this when you know you're targeting .NET Framework 2.0/3.x (CLR 2.0).
///
/// # Safety
///
/// This function loads a DLL and calls an exported function.
pub unsafe fn get_jit_compiler_clr2() -> Option<*mut ICorJitCompiler> {
    unsafe { get_jit_from_dll(w!("mscorjit.dll")) }
}

/// Internal helper to load JIT from a specific DLL.
///
/// Tries `GetModuleHandleW` first (for single-file apps where the module is already loaded),
/// then falls back to `LoadLibraryW`.
unsafe fn get_jit_from_dll(dll_name: windows::core::PCWSTR) -> Option<*mut ICorJitCompiler> {
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
    use windows::core::s;

    unsafe {
        // Try GetModuleHandle first - works for single-file apps where clrjit is already loaded
        let module = GetModuleHandleW(dll_name)
            .ok()
            // Fall back to LoadLibrary for standalone DLL scenarios
            .or_else(|| LoadLibraryW(dll_name).ok())?;

        let proc = GetProcAddress(module, s!("getJit"))?;
        let get_jit_fn: extern "C" fn() -> *mut ICorJitCompiler = std::mem::transmute(proc);
        let jit = get_jit_fn();

        if jit.is_null() {
            None
        } else {
            Some(jit)
        }
    }
}

