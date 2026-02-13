//! IXCLRDataAppDomain interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IXCLRDataAppDomain - Represents an application domain.
#[interface("7CA04601-C702-4670-A63C-FA44F7DA7BD5")]
pub unsafe trait IXCLRDataAppDomain: IUnknown {
    /// Get the owning process.
    pub unsafe fn GetProcess(&self, process: *mut *mut IUnknown) -> HRESULT;

    /// Get the name of this app domain.
    pub unsafe fn GetName(&self, bufLen: u32, nameLen: *mut u32, name: *mut u16) -> HRESULT;

    /// Get the unique identifier for this app domain.
    pub unsafe fn GetUniqueID(&self, id: *mut u64) -> HRESULT;

    /// Get the flags for this app domain.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether the given interface represents the same target state.
    pub unsafe fn IsSameObject(&self, appDomain: *mut IXCLRDataAppDomain) -> HRESULT;

    /// Get the managed object representing this app domain.
    pub unsafe fn GetManagedObject(&self, value: *mut *mut IUnknown) -> HRESULT;
}
