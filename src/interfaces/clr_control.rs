//! ICLRControl and related hosting control interfaces.
//!
//! These interfaces allow the host to control CLR behavior.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// ICLRControl - Allows the host to get references to managers and configure them.
#[interface("9065597E-D1A1-4FB2-B6BA-7E1FCE230F61")]
pub unsafe trait ICLRControl: IUnknown {
    /// Gets a reference to a CLR manager.
    pub unsafe fn GetCLRManager(
        &self,
        riid: *const GUID,
        ppObject: *mut *mut c_void,
    ) -> HRESULT;

    /// Sets application domain manager type.
    pub unsafe fn SetAppDomainManagerType(
        &self,
        pwzAppDomainManagerAssembly: *const u16,
        pwzAppDomainManagerType: *const u16,
    ) -> HRESULT;
}

/// IHostControl - Implemented by the host to provide callbacks to the CLR.
#[interface("02CA073C-7079-4860-880A-C2F7A449C991")]
pub unsafe trait IHostControl: IUnknown {
    /// Gets a reference to a host manager.
    pub unsafe fn GetHostManager(
        &self,
        riid: *const GUID,
        ppObject: *mut *mut c_void,
    ) -> HRESULT;

    /// Sets the application domain manager.
    pub unsafe fn SetAppDomainManager(
        &self,
        dwAppDomainID: u32,
        pUnkAppDomainManager: *mut IUnknown,
    ) -> HRESULT;
}

/// ICLRGCManager - GC control interface.
#[interface("54D9007E-A8E2-4885-B7BF-F998DEEE4F2A")]
pub unsafe trait ICLRGCManager: IUnknown {
    /// Collect garbage for specified generations.
    pub unsafe fn Collect(&self, Generation: i32) -> HRESULT;

    /// Get GC statistics.
    pub unsafe fn GetStats(&self, pStats: *mut c_void) -> HRESULT;

    /// Set GC startup limits.
    pub unsafe fn SetGCStartupLimits(
        &self,
        SegmentSize: u32,
        MaxGen0Size: u32,
    ) -> HRESULT;
}

/// ICLRGCManager2 - Extended GC control interface.
#[interface("0603B793-A97A-4712-9CB4-0CD1C74C0F7C")]
pub unsafe trait ICLRGCManager2: IUnknown {
    // ICLRGCManager methods
    pub unsafe fn Collect(&self, Generation: i32) -> HRESULT;
    pub unsafe fn GetStats(&self, pStats: *mut c_void) -> HRESULT;
    pub unsafe fn SetGCStartupLimits(&self, SegmentSize: u32, MaxGen0Size: u32) -> HRESULT;

    // ICLRGCManager2 specific
    /// Set GC startup limits with 64-bit sizes.
    pub unsafe fn SetGCStartupLimitsEx(
        &self,
        SegmentSize: u64,
        MaxGen0Size: u64,
    ) -> HRESULT;
}

/// ICLRErrorReportingManager - Error reporting control.
#[interface("980D2F1A-BF79-4C08-812A-BB9778928F78")]
pub unsafe trait ICLRErrorReportingManager: IUnknown {
    /// Get bucket parameters for Watson reporting.
    pub unsafe fn GetBucketParametersForCurrentException(
        &self,
        pParams: *mut c_void,
    ) -> HRESULT;

    /// Begin custom Watson dump.
    pub unsafe fn BeginCustomDump(
        &self,
        dwFlavor: u32,
        dwNumItems: u32,
        items: *const c_void,
        dwReserved: u32,
    ) -> HRESULT;

    /// End custom Watson dump.
    pub unsafe fn EndCustomDump(&self) -> HRESULT;
}

/// ICLRPolicyManager - Runtime policy control.
#[interface("7D466EC7-1E6E-4A03-8F77-CF951BCEA2D7")]
pub unsafe trait ICLRPolicyManager: IUnknown {
    /// Set default action for a CLR failure.
    pub unsafe fn SetDefaultAction(
        &self,
        operation: u32,
        action: u32,
    ) -> HRESULT;

    /// Set timeout for an operation.
    pub unsafe fn SetTimeout(
        &self,
        operation: u32,
        dwMilliseconds: u32,
    ) -> HRESULT;

    /// Set action on a failure type.
    pub unsafe fn SetActionOnFailure(
        &self,
        failure: u32,
        action: u32,
    ) -> HRESULT;

    /// Set unhandled exception policy.
    pub unsafe fn SetUnhandledExceptionPolicy(
        &self,
        policy: u32,
    ) -> HRESULT;
}

/// ICLRHostProtectionManager - Code access security control.
#[interface("89F25F5C-CEEF-43E1-9CFA-A68CE863AAAC")]
pub unsafe trait ICLRHostProtectionManager: IUnknown {
    /// Set host protection categories.
    pub unsafe fn SetProtectedCategories(
        &self,
        categories: u32,
    ) -> HRESULT;

    /// Set eager serialize grant sets.
    pub unsafe fn SetEagerSerializeGrantSets(&self) -> HRESULT;
}

