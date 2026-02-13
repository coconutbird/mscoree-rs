//! ICLRMetaHost interface definition.

use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, PCWSTR, interface};

/// ICLRMetaHost interface for CLR hosting (.NET 4.0+).
///
/// This is the main entry point for discovering and loading CLR versions.
/// Obtain an instance using `CLRCreateInstance` with `CLSID_CLRMetaHost`.
#[interface("D332DB9E-B9B3-4125-8207-A14884F53216")]
pub unsafe trait ICLRMetaHost: IUnknown {
    /// Gets the ICLRRuntimeInfo interface for a specific CLR version.
    ///
    /// # Arguments
    ///
    /// * `pwzVersion` - The CLR version string (e.g., "v4.0.30319")
    /// * `riid` - The IID of the interface to return (typically IID_ICLRRuntimeInfo)
    /// * `ppRuntime` - Receives the requested interface
    pub unsafe fn GetRuntime(
        &self,
        pwzVersion: PCWSTR,
        riid: *const GUID,
        ppRuntime: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the CLR version that was used to compile an assembly.
    ///
    /// # Arguments
    ///
    /// * `pwzFilePath` - Path to the assembly file
    /// * `pwzBuffer` - Buffer to receive the version string
    /// * `pcchBuffer` - Size of buffer in characters
    pub unsafe fn GetVersionFromFile(
        &self,
        pwzFilePath: PCWSTR,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
    ) -> HRESULT;

    /// Enumerates all installed CLR versions.
    ///
    /// # Arguments
    ///
    /// * `ppEnumerator` - Receives an IEnumUnknown that enumerates ICLRRuntimeInfo
    pub unsafe fn EnumerateInstalledRuntimes(
        &self,
        ppEnumerator: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Enumerates all CLR versions loaded in a process.
    ///
    /// # Arguments
    ///
    /// * `hndProcess` - Handle to the process
    /// * `ppEnumerator` - Receives an IEnumUnknown that enumerates ICLRRuntimeInfo
    pub unsafe fn EnumerateLoadedRuntimes(
        &self,
        hndProcess: *mut core::ffi::c_void,
        ppEnumerator: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Registers a callback for runtime load notifications.
    pub unsafe fn RequestRuntimeLoadedNotification(
        &self,
        pCallbackFunction: *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Queries for legacy v2 runtime binding.
    pub unsafe fn QueryLegacyV2RuntimeBinding(
        &self,
        riid: *const GUID,
        ppUnk: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gracefully shuts down all loaded runtimes and exits the process.
    pub unsafe fn ExitProcess(&self, iExitCode: i32) -> HRESULT;
}
