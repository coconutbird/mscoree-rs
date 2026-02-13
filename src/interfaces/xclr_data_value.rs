//! IXCLRDataValue interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::clr_data_target::CLRDATA_ADDRESS;
use super::xclr_data_process::CLRDATA_ENUM;

/// IXCLRDataValue - Represents a CLR value.
#[interface("96EC93C7-1000-4E93-8991-98D8766E6666")]
pub unsafe trait IXCLRDataValue: IUnknown {
    /// Get the flags for this value.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Get the address of this value.
    pub unsafe fn GetAddress(&self, address: *mut CLRDATA_ADDRESS) -> HRESULT;

    /// Get the size of this value.
    pub unsafe fn GetSize(&self, size: *mut u64) -> HRESULT;

    /// Read bytes from this value.
    pub unsafe fn GetBytes(&self, bufLen: u32, dataSize: *mut u32, buffer: *mut u8) -> HRESULT;

    /// Write bytes to this value.
    pub unsafe fn SetBytes(&self, bufLen: u32, dataSize: *mut u32, buffer: *const u8) -> HRESULT;

    /// Get the type of this value.
    pub unsafe fn GetType(&self, typeInstance: *mut *mut IUnknown) -> HRESULT;

    /// Get the number of fields in this value.
    pub unsafe fn GetNumFields(&self, numFields: *mut u32) -> HRESULT;

    /// Get a field by index.
    pub unsafe fn GetFieldByIndex(
        &self,
        index: u32,
        field: *mut *mut IXCLRDataValue,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
        token: *mut u32,
    ) -> HRESULT;

    /// Generic request operation.
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;

    /// Begin enumeration of fields.
    pub unsafe fn StartEnumFields(
        &self,
        flags: u32,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next field.
    pub unsafe fn EnumField(
        &self,
        handle: *mut CLRDATA_ENUM,
        field: *mut *mut IXCLRDataValue,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
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
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next field by name.
    pub unsafe fn EnumFieldByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        field: *mut *mut IXCLRDataValue,
        token: *mut u32,
    ) -> HRESULT;

    /// End enumeration of fields by name.
    pub unsafe fn EndEnumFieldsByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get a field by token.
    pub unsafe fn GetFieldByToken(
        &self,
        token: u32,
        field: *mut *mut IXCLRDataValue,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
    ) -> HRESULT;

    /// Get the associated value (e.g. boxed value).
    pub unsafe fn GetAssociatedValue(&self, assocValue: *mut *mut IXCLRDataValue) -> HRESULT;

    /// Get the associated type.
    pub unsafe fn GetAssociatedType(&self, assocType: *mut *mut IUnknown) -> HRESULT;

    /// Get a string value.
    pub unsafe fn GetString(&self, bufLen: u32, strLen: *mut u32, str: *mut u16) -> HRESULT;

    /// Get array properties.
    pub unsafe fn GetArrayProperties(
        &self,
        rank: *mut u32,
        totalElements: *mut u32,
        dims: u32,
        dimSizes: *mut u32,
        bases: u32,
        dimBases: *mut i32,
    ) -> HRESULT;

    /// Get an array element.
    pub unsafe fn GetArrayElement(
        &self,
        numIndices: u32,
        indices: *const i32,
        element: *mut *mut IXCLRDataValue,
    ) -> HRESULT;

    /// Get number of memory locations.
    pub unsafe fn GetNumLocations(&self, numLocs: *mut u32) -> HRESULT;

    /// Get a memory location by index.
    pub unsafe fn GetLocationByIndex(
        &self,
        index: u32,
        flags: *mut u32,
        address: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
}
