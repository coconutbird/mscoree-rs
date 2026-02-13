//! IXCLRDataAssembly interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::xclr_data_process::CLRDATA_ENUM;

/// IXCLRDataAssembly - Represents an assembly.
#[interface("2FA17588-43C2-46AB-9B51-C8F01E39C9AC")]
pub unsafe trait IXCLRDataAssembly: IUnknown {
    /// Begin enumeration of modules in this assembly.
    pub unsafe fn StartEnumModules(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next module in the enumeration.
    pub unsafe fn EnumModule(
        &self,
        handle: *mut CLRDATA_ENUM,
        module: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of modules.
    pub unsafe fn EndEnumModules(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the name of this assembly.
    pub unsafe fn GetName(
        &self,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the file name of this assembly.
    pub unsafe fn GetFileName(
        &self,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the flags for this assembly.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether the given interface represents the same target state.
    pub unsafe fn IsSameObject(&self, assembly: *mut IXCLRDataAssembly) -> HRESULT;

    /// Generic request operation.
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;

    /// Begin enumeration of app domains containing this assembly.
    pub unsafe fn StartEnumAppDomains(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next app domain in the enumeration.
    pub unsafe fn EnumAppDomain(
        &self,
        handle: *mut CLRDATA_ENUM,
        appDomain: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of app domains.
    pub unsafe fn EndEnumAppDomains(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the display name of this assembly.
    pub unsafe fn GetDisplayName(
        &self,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;
}

