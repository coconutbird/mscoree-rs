//! IXCLRDataTypeDefinition and IXCLRDataTypeInstance interfaces.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::xclr_data_process::CLRDATA_ENUM;

/// IXCLRDataTypeDefinition - Represents a type definition.
#[interface("4675666C-C275-45b8-9F6C-AB165D5C1E09")]
pub unsafe trait IXCLRDataTypeDefinition: IUnknown {
    /// Get the module containing this type.
    pub unsafe fn GetModule(&self, module: *mut *mut IUnknown) -> HRESULT;

    /// Begin enumeration of method definitions.
    pub unsafe fn StartEnumMethodDefinitions(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next method definition.
    pub unsafe fn EnumMethodDefinition(
        &self,
        handle: *mut CLRDATA_ENUM,
        methodDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of method definitions.
    pub unsafe fn EndEnumMethodDefinitions(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of method definitions by name.
    pub unsafe fn StartEnumMethodDefinitionsByName(
        &self,
        flags: u32,
        name: *const u16,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next method definition by name.
    pub unsafe fn EnumMethodDefinitionByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        methodDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of method definitions by name.
    pub unsafe fn EndEnumMethodDefinitionsByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of type instances.
    pub unsafe fn StartEnumInstances(
        &self,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next type instance.
    pub unsafe fn EnumInstance(
        &self,
        handle: *mut CLRDATA_ENUM,
        instance: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of type instances.
    pub unsafe fn EndEnumInstances(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the name of this type.
    pub unsafe fn GetName(
        &self,
        flags: u32,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
    ) -> HRESULT;

    /// Get the token and scope for this type.
    pub unsafe fn GetTokenAndScope(&self, token: *mut u32, module: *mut *mut IUnknown) -> HRESULT;

    /// Get the correlation ID.
    pub unsafe fn GetCorElementType(&self, r#type: *mut u32) -> HRESULT;

    /// Get the flags for this type.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether this is the same object as another.
    pub unsafe fn IsSameObject(&self, r#type: *mut IXCLRDataTypeDefinition) -> HRESULT;

    /// Generic request operation.
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;

    /// Get array fixed rank size.
    pub unsafe fn GetArrayFixedSize(&self, size: *mut u32) -> HRESULT;

    /// Get the base type.
    pub unsafe fn GetBase(&self, base: *mut *mut IXCLRDataTypeDefinition) -> HRESULT;

    /// Get the number of fields.
    pub unsafe fn GetNumFields(&self, flags: u32, numFields: *mut u32) -> HRESULT;

    /// Begin enumeration of fields.
    pub unsafe fn StartEnumFields(&self, flags: u32, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next field.
    pub unsafe fn EnumField(
        &self,
        handle: *mut CLRDATA_ENUM,
        nameBuf: *mut u16,
        bufLen: u32,
        nameLen: *mut u32,
        r#type: *mut *mut IXCLRDataTypeDefinition,
        flags: *mut u32,
        token: *mut u32,
    ) -> HRESULT;

    /// End enumeration of fields.
    pub unsafe fn EndEnumFields(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of fields by name.
    pub unsafe fn StartEnumFieldsByName(
        &self,
        name: *const u16,
        nameFlags: u32,
        fieldFlags: u32,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;
}
