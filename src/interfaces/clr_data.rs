//! CLR Data interfaces for memory enumeration and metadata location.
//!
//! These interfaces are used for memory dump generation and metadata access.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, PCWSTR, interface};

use super::data_target::CLRDATA_ADDRESS;

/// Flags for controlling which memory regions are enumerated.
pub mod CLRDataEnumMemoryFlags {
    pub const CLRDATA_ENUM_MEM_DEFAULT: u32 = 0x0;
    pub const CLRDATA_ENUM_MEM_MINI: u32 = 0x0;
    pub const CLRDATA_ENUM_MEM_HEAP: u32 = 0x1;
    pub const CLRDATA_ENUM_MEM_TRIAGE: u32 = 0x2;
    pub const CLRDATA_ENUM_MEM_HEAP2: u32 = 0x3;
}

/// ICLRDataEnumMemoryRegionsCallback - Callback for memory region enumeration.
#[interface("BCDD6908-BA2D-4ec5-96CF-DF4D5CDCB4A4")]
pub unsafe trait ICLRDataEnumMemoryRegionsCallback: IUnknown {
    /// Called for every memory region enumerated.
    pub unsafe fn EnumMemoryRegion(&self, address: CLRDATA_ADDRESS, size: u32) -> HRESULT;
}

/// ICLRDataEnumMemoryRegionsCallback2 - Extended callback with memory update.
#[interface("3721A26F-8B91-4D98-A388-DB17B356FADB")]
pub unsafe trait ICLRDataEnumMemoryRegionsCallback2:
    ICLRDataEnumMemoryRegionsCallback
{
    /// Called to overwrite/poison memory regions with specified data.
    pub unsafe fn UpdateMemoryRegion(
        &self,
        address: CLRDATA_ADDRESS,
        bufferSize: u32,
        buffer: *const u8,
    ) -> HRESULT;
}

/// ICLRDataLoggingCallback - Optional callback for logging operations and errors.
#[interface("F315248D-8B79-49DB-B184-37426559F703")]
pub unsafe trait ICLRDataLoggingCallback: IUnknown {
    pub unsafe fn LogMessage(&self, message: *const i8) -> HRESULT;
}

/// ICLRDataEnumMemoryRegions - Memory enumeration interface.
/// This is one of the top-level interfaces creatable by CLRDataCreateInstance.
#[interface("471c35b4-7c2f-4ef0-a945-00f8c38056f1")]
pub unsafe trait ICLRDataEnumMemoryRegions: IUnknown {
    /// Enumerates regions of interest as specified by the flags argument.
    /// Use constants from CLRDataEnumMemoryFlags module for clrFlags parameter.
    pub unsafe fn EnumMemoryRegions(
        &self,
        callback: *mut ICLRDataEnumMemoryRegionsCallback,
        miniDumpFlags: u32,
        clrFlags: u32,
    ) -> HRESULT;
}

/// ICLRRuntimeLocator - Interface to locate the runtime module.
#[interface("b760bf44-9377-4597-8be7-58083bdc5146")]
pub unsafe trait ICLRRuntimeLocator: IUnknown {
    /// Returns the base address of the module containing the runtime.
    pub unsafe fn GetRuntimeBase(&self, baseAddress: *mut CLRDATA_ADDRESS) -> HRESULT;
}

/// ICLRMetadataLocator - Interface to locate metadata of assemblies.
#[interface("aa8fa804-bc05-4642-b2c5-c353ed22fc63")]
pub unsafe trait ICLRMetadataLocator: IUnknown {
    /// Ask the target to retrieve metadata for an image.
    pub unsafe fn GetMetadata(
        &self,
        imagePath: PCWSTR,
        imageTimestamp: u32,
        imageSize: u32,
        mvid: *const GUID,
        mdRva: u32,
        flags: u32,
        bufferSize: u32,
        buffer: *mut u8,
        dataSize: *mut u32,
    ) -> HRESULT;
}

/// IXCLRDataTarget3 - Extended data target with metadata recovery.
/// Note: This extends ICLRDataTarget2 but is defined standalone here.
#[interface("59d9b5e1-4a6f-4531-84c3-51d12da22fd4")]
pub unsafe trait IXCLRDataTarget3: IUnknown {
    /// Ask the target to recover metadata for an image.
    pub unsafe fn GetMetaData(
        &self,
        imagePath: PCWSTR,
        imageTimestamp: u32,
        imageSize: u32,
        mvid: *const GUID,
        mdRva: u32,
        flags: u32,
        bufferSize: u32,
        buffer: *mut u8,
        dataSize: *mut u32,
    ) -> HRESULT;
}
