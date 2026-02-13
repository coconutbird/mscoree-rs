//! ICLRDebugging and related debugging interfaces.
//!
//! These interfaces are used to attach debuggers to CLR processes.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface, PCWSTR};

/// CLR_DEBUGGING_VERSION structure.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CLR_DEBUGGING_VERSION {
    pub wStructVersion: u16,
    pub wMajor: u16,
    pub wMinor: u16,
    pub wBuild: u16,
    pub wRevision: u16,
}

/// CLR_DEBUGGING_PROCESS_FLAGS.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CLR_DEBUGGING_PROCESS_FLAGS {
    CLR_DEBUGGING_MANAGED_EVENT_PENDING = 1,
    CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH = 2,
}

/// ICLRDebugging - Main interface for debugging CLR processes.
#[interface("D28F3C5A-9634-4206-A509-477552EEFB10")]
pub unsafe trait ICLRDebugging: IUnknown {
    /// Open a virtual process for debugging.
    pub unsafe fn OpenVirtualProcess(
        &self,
        moduleBaseAddress: u64,
        pDataTarget: *mut IUnknown,
        pLibraryProvider: *mut ICLRDebuggingLibraryProvider,
        pMaxDebuggerSupportedVersion: *const CLR_DEBUGGING_VERSION,
        riidProcess: *const GUID,
        ppProcess: *mut *mut IUnknown,
        pVersion: *mut CLR_DEBUGGING_VERSION,
        pdwFlags: *mut u32,
    ) -> HRESULT;

    /// Check if a module is a CLR module.
    pub unsafe fn CanUnloadNow(&self, hModule: *mut c_void) -> HRESULT;
}

/// ICLRDebuggingLibraryProvider - Provides debugging libraries.
#[interface("3151C08D-4D09-4F9B-8838-2880BF18FE51")]
pub unsafe trait ICLRDebuggingLibraryProvider: IUnknown {
    /// Provide library for debugging.
    pub unsafe fn ProvideLibrary(
        &self,
        pwszFileName: PCWSTR,
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        phModule: *mut *mut c_void,
    ) -> HRESULT;
}

/// ICLRDebuggingLibraryProvider2 - Extended library provider.
#[interface("E04E2FF1-DCFD-45D5-BCD1-16FFF2FAF7BA")]
pub unsafe trait ICLRDebuggingLibraryProvider2: IUnknown {
    // ICLRDebuggingLibraryProvider method
    pub unsafe fn ProvideLibrary(
        &self,
        pwszFileName: PCWSTR,
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        phModule: *mut *mut c_void,
    ) -> HRESULT;

    // ICLRDebuggingLibraryProvider2 method
    /// Provide library with MVID/index.
    pub unsafe fn ProvideLibrary2(
        &self,
        pwszFileName: PCWSTR,
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        pLibraryId: *const GUID,
        pLibraryIndexId: *const GUID,
        phModule: *mut *mut c_void,
    ) -> HRESULT;
}

/// ICLRDebuggingLibraryProvider3 - Extended library provider with Windows debugger support.
#[interface("DE3AAB18-46A0-48B4-BF0D-2C336E69EA1B")]
pub unsafe trait ICLRDebuggingLibraryProvider3: IUnknown {
    // ICLRDebuggingLibraryProvider method
    pub unsafe fn ProvideLibrary(
        &self,
        pwszFileName: PCWSTR,
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        phModule: *mut *mut c_void,
    ) -> HRESULT;

    // ICLRDebuggingLibraryProvider2 method
    pub unsafe fn ProvideLibrary2(
        &self,
        pwszFileName: PCWSTR,
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        pLibraryId: *const GUID,
        pLibraryIndexId: *const GUID,
        phModule: *mut *mut c_void,
    ) -> HRESULT;

    // ICLRDebuggingLibraryProvider3 method
    /// Provide Windows debugger library.
    pub unsafe fn ProvideWindowsLibrary(
        &self,
        pwszFileName: PCWSTR,
        pwszRuntimeModule: PCWSTR,
        indexType: u32,  // LIBRARY_PROVIDER_INDEX_TYPE
        dwTimestamp: u32,
        dwSizeOfImage: u32,
        ppResolvedModulePath: *mut PCWSTR,
    ) -> HRESULT;
}

