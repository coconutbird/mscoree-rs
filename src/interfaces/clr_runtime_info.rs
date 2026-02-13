//! ICLRRuntimeInfo interface definition.

use windows::core::{BOOL, GUID, HRESULT, IUnknown, IUnknown_Vtbl, PCWSTR, interface};

/// ICLRRuntimeInfo interface for querying CLR version information.
///
/// Provides information about a specific CLR version and allows getting
/// hosting interfaces for that runtime.
#[interface("BD39D1D2-BA2F-486A-89B0-B4B0CB466891")]
pub unsafe trait ICLRRuntimeInfo: IUnknown {
    /// Gets the version string for this runtime.
    ///
    /// # Arguments
    ///
    /// * `pwzBuffer` - Buffer to receive the version string
    /// * `pcchBuffer` - Size of buffer in characters
    pub unsafe fn GetVersionString(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT;

    /// Gets the directory where this runtime is installed.
    ///
    /// # Arguments
    ///
    /// * `pwzBuffer` - Buffer to receive the directory path
    /// * `pcchBuffer` - Size of buffer in characters
    pub unsafe fn GetRuntimeDirectory(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT;

    /// Checks if this runtime is loaded in the specified process.
    ///
    /// # Arguments
    ///
    /// * `hndProcess` - Handle to the process to check
    /// * `pbLoaded` - Receives TRUE if loaded, FALSE otherwise
    pub unsafe fn IsLoaded(
        &self,
        hndProcess: *mut core::ffi::c_void,
        pbLoaded: *mut BOOL,
    ) -> HRESULT;

    /// Translates an HRESULT to an error message.
    ///
    /// # Arguments
    ///
    /// * `iResourceID` - The HRESULT to translate
    /// * `pwzBuffer` - Buffer to receive the error message
    /// * `pcchBuffer` - Size of buffer in characters
    /// * `iLocaleID` - Locale ID for the message
    pub unsafe fn LoadErrorString(
        &self,
        iResourceID: u32,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
        iLocaleID: i32,
    ) -> HRESULT;

    /// Loads a library from the runtime directory.
    ///
    /// # Arguments
    ///
    /// * `pwzDllName` - Name of the DLL to load
    /// * `phndModule` - Receives the module handle
    pub unsafe fn LoadLibrary(
        &self,
        pwzDllName: PCWSTR,
        phndModule: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the address of a function exported by this runtime.
    ///
    /// # Arguments
    ///
    /// * `pszProcName` - Name of the function
    /// * `ppProc` - Receives the function address
    pub unsafe fn GetProcAddress(
        &self,
        pszProcName: *const i8,
        ppProc: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets a hosting interface for this runtime.
    ///
    /// # Arguments
    ///
    /// * `rclsid` - CLSID of the coclass (e.g., CLSID_CLRRuntimeHost)
    /// * `riid` - IID of the interface (e.g., IID_ICLRRuntimeHost)
    /// * `ppUnk` - Receives the interface pointer
    pub unsafe fn GetInterface(
        &self,
        rclsid: *const GUID,
        riid: *const GUID,
        ppUnk: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Checks if this runtime can be loaded in-process.
    pub unsafe fn IsLoadable(&self, pbLoadable: *mut BOOL) -> HRESULT;

    /// Sets the default startup flags for this runtime.
    pub unsafe fn SetDefaultStartupFlags(
        &self,
        dwStartupFlags: u32,
        pwzHostConfigFile: PCWSTR,
    ) -> HRESULT;

    /// Gets the default startup flags for this runtime.
    pub unsafe fn GetDefaultStartupFlags(
        &self,
        pdwStartupFlags: *mut u32,
        pwzHostConfigFile: *mut u16,
        pcchHostConfigFile: *mut u32,
    ) -> HRESULT;

    /// Binds this runtime as the legacy v2 runtime.
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> HRESULT;

    /// Checks if this runtime is started.
    pub unsafe fn IsStarted(&self, pbStarted: *mut BOOL, pdwStartupFlags: *mut u32) -> HRESULT;
}
