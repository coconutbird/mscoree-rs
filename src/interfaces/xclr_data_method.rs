//! IXCLRDataMethodDefinition and IXCLRDataMethodInstance interfaces.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::data_target::CLRDATA_ADDRESS;
use super::xclr_data_process::CLRDATA_ENUM;
use super::xclr_data_types::CLRDATA_METHDEF_EXTENT;

/// IXCLRDataMethodDefinition - Represents a method definition.
#[interface("AAF60008-FB2C-420B-8FB1-42D244A54A97")]
pub unsafe trait IXCLRDataMethodDefinition: IUnknown {
    /// Get the type definition that contains this method.
    pub unsafe fn GetTypeDefinition(
        &self,
        typeDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Begin enumeration of method instances.
    pub unsafe fn StartEnumInstances(
        &self,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next method instance in the enumeration.
    pub unsafe fn EnumInstance(
        &self,
        handle: *mut CLRDATA_ENUM,
        instance: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of method instances.
    pub unsafe fn EndEnumInstances(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the name of this method.
    pub unsafe fn GetName(
        &self,
        flags: u32,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the token and scope for this method.
    pub unsafe fn GetTokenAndScope(
        &self,
        token: *mut u32,
        module: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get flags for this method.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether this is the same object as another.
    pub unsafe fn IsSameObject(&self, method: *mut IXCLRDataMethodDefinition) -> HRESULT;

    /// Get the latest Edit and Continue version.
    pub unsafe fn GetLatestEnCVersion(&self, version: *mut u32) -> HRESULT;

    /// Begin enumeration of code extents.
    pub unsafe fn StartEnumExtents(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next code extent.
    pub unsafe fn EnumExtent(
        &self,
        handle: *mut CLRDATA_ENUM,
        extent: *mut CLRDATA_METHDEF_EXTENT,
    ) -> HRESULT;

    /// End enumeration of code extents.
    pub unsafe fn EndEnumExtents(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Generic request operation.
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;

    /// Get code notification flags.
    pub unsafe fn GetCodeNotification(&self, flags: *mut u32) -> HRESULT;

    /// Set code notification flags.
    pub unsafe fn SetCodeNotification(&self, flags: u32) -> HRESULT;

    /// Get representative entry address.
    pub unsafe fn GetRepresentativeEntryAddress(
        &self,
        address: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Check if this method has body.
    pub unsafe fn HasClassOrMethodInstantiation(&self, bGeneric: *mut i32) -> HRESULT;
}

