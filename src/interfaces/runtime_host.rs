//! ICLRRuntimeHost interface definition.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, PCWSTR, interface};

/// Callback function type for ExecuteInAppDomain.
pub type FExecuteInAppDomainCallback =
    unsafe extern "system" fn(cookie: *mut core::ffi::c_void) -> HRESULT;

/// ICLRRuntimeHost interface for hosting the CLR (.NET 2.0+).
///
/// This interface provides methods to start/stop the CLR and execute managed code.
#[interface("90F1A06C-7712-4762-86B5-7A5EBA6BDB02")]
pub unsafe trait ICLRRuntimeHost: IUnknown {
    /// Initializes the CLR into a process.
    pub unsafe fn Start(&self) -> HRESULT;

    /// Stops the execution of code by the runtime.
    pub unsafe fn Stop(&self) -> HRESULT;

    /// Sets the host control interface.
    ///
    /// Must be called before Start().
    pub unsafe fn SetHostControl(&self, pHostControl: *mut core::ffi::c_void) -> HRESULT;

    /// Gets the CLR control interface.
    pub unsafe fn GetCLRControl(&self, pCLRControl: *mut *mut core::ffi::c_void) -> HRESULT;

    /// Unloads an AppDomain by its ID.
    pub unsafe fn UnloadAppDomain(&self, dwAppDomainId: u32, fWaitUntilDone: i32) -> HRESULT;

    /// Executes a method in a specific AppDomain.
    pub unsafe fn ExecuteInAppDomain(
        &self,
        dwAppDomainId: u32,
        pCallback: FExecuteInAppDomainCallback,
        cookie: *mut core::ffi::c_void,
    ) -> HRESULT;

    /// Gets the current AppDomain ID.
    pub unsafe fn GetCurrentAppDomainId(&self, pdwAppDomainId: *mut u32) -> HRESULT;

    /// Executes a method in the default AppDomain.
    ///
    /// # Arguments
    ///
    /// * `pwzAssemblyPath` - Path to the assembly
    /// * `pwzTypeName` - Full name of the type containing the method
    /// * `pwzMethodName` - Name of the static method to invoke
    /// * `pwzArgument` - String argument to pass to the method
    /// * `pReturnValue` - Receives the return value from the method
    ///
    /// # Notes
    ///
    /// The method must have the signature:
    /// `static int MethodName(string argument)`
    pub unsafe fn ExecuteInDefaultAppDomain(
        &self,
        pwzAssemblyPath: PCWSTR,
        pwzTypeName: PCWSTR,
        pwzMethodName: PCWSTR,
        pwzArgument: PCWSTR,
        pReturnValue: *mut u32,
    ) -> HRESULT;

    /// Used for manifest-based ClickOnce deployment.
    pub unsafe fn ExecuteApplication(
        &self,
        pwzAppFullName: PCWSTR,
        dwManifestPaths: u32,
        ppwzManifestPaths: *const PCWSTR,
        dwActivationData: u32,
        ppwzActivationData: *const PCWSTR,
        pReturnValue: *mut i32,
    ) -> HRESULT;
}
