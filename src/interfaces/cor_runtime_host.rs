//! ICorRuntimeHost interface definition (legacy .NET 1.x hosting).

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// ICorRuntimeHost interface for legacy CLR hosting (.NET 1.x).
///
/// This interface is superseded by ICLRRuntimeHost in .NET 2.0+,
/// but is still useful for working with AppDomains.
#[interface("CB2F6722-AB3A-11D2-9C40-00C04FA30A3E")]
pub unsafe trait ICorRuntimeHost: IUnknown {
    /// Creates a logical thread state. Do not use.
    pub unsafe fn CreateLogicalThreadState(&self) -> HRESULT;

    /// Deletes a logical thread state. Do not use.
    pub unsafe fn DeleteLogicalThreadState(&self) -> HRESULT;

    /// Switches in logical thread state. Do not use.
    pub unsafe fn SwitchInLogicalThreadState(
        &self,
        pFiberCookie: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Switches out logical thread state. Do not use.
    pub unsafe fn SwitchOutLogicalThreadState(
        &self,
        pFiberCookie: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the number of locks held by the current logical thread. Do not use.
    pub unsafe fn LocksHeldByLogicalThread(&self, pCount: *mut u32) -> HRESULT;

    /// Maps a file into memory. Obsolete.
    pub unsafe fn MapFile(
        &self,
        hFile: *mut core::ffi::c_void,
        hMapAddress: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the configuration object.
    pub unsafe fn GetConfiguration(&self, pConfiguration: *mut *mut core::ffi::c_void) -> HRESULT;

    /// Starts the CLR.
    pub unsafe fn Start(&self) -> HRESULT;

    /// Stops the CLR.
    pub unsafe fn Stop(&self) -> HRESULT;

    /// Creates a new AppDomain.
    ///
    /// # Arguments
    ///
    /// * `pwzFriendlyName` - Friendly name for the domain
    /// * `pIdentityArray` - Evidence for the domain (can be null)
    /// * `pAppDomain` - Receives an _AppDomain interface pointer
    pub unsafe fn CreateDomain(
        &self,
        pwzFriendlyName: *const u16,
        pIdentityArray: *mut core::ffi::c_void,
        pAppDomain: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the default AppDomain.
    pub unsafe fn GetDefaultDomain(&self, pAppDomain: *mut *mut core::ffi::c_void) -> HRESULT;

    /// Begins enumerating AppDomains.
    pub unsafe fn EnumDomains(&self, hEnum: *mut *mut core::ffi::c_void) -> HRESULT;

    /// Gets the next AppDomain in the enumeration.
    pub unsafe fn NextDomain(
        &self,
        hEnum: *mut core::ffi::c_void,
        pAppDomain: *mut *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Closes an AppDomain enumeration.
    pub unsafe fn CloseEnum(&self, hEnum: *mut core::ffi::c_void) -> HRESULT;

    /// Creates an IAppDomainSetup object.
    pub unsafe fn CreateDomainSetup(&self, pAppDomainSetup: *mut *mut core::ffi::c_void)
    -> HRESULT;

    /// Creates evidence for an AppDomain.
    pub unsafe fn CreateEvidence(&self, pEvidence: *mut *mut core::ffi::c_void) -> HRESULT;

    /// Unloads an AppDomain.
    pub unsafe fn UnloadDomain(&self, pAppDomain: *mut core::ffi::c_void) -> HRESULT;

    /// Gets the current AppDomain for the current thread.
    pub unsafe fn CurrentDomain(&self, pAppDomain: *mut *mut core::ffi::c_void) -> HRESULT;
}

/// Creates a new AppDomain with extended configuration.
///
/// # Arguments
///
/// * `pwzFriendlyName` - Friendly name for the domain
/// * `pSetup` - IAppDomainSetup configuration
/// * `pEvidence` - Security evidence (can be null)
/// * `pAppDomain` - Receives an _AppDomain interface pointer
#[interface("CB2F6722-AB3A-11D2-9C40-00C04FA30A3E")]
pub unsafe trait ICorRuntimeHostExt: ICorRuntimeHost {
    /// Creates a domain with extended setup options.
    pub unsafe fn CreateDomainEx(
        &self,
        pwzFriendlyName: *const u16,
        pSetup: *mut core::ffi::c_void,
        pEvidence: *mut core::ffi::c_void,
        pAppDomain: *mut *mut core::ffi::c_void,
    ) -> HRESULT;
}
