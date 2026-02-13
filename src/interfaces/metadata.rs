//! IMetaData interface definitions for reading and writing assembly metadata.
//!
//! These interfaces provide access to .NET assembly metadata (types, methods, fields, etc.)

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IMetaDataDispenser - Creates metadata import/emit scopes.
#[interface("809C652E-7396-11D2-9771-00A0C9B4D50C")]
pub unsafe trait IMetaDataDispenser: IUnknown {
    /// Define the scope of a new metadata.
    pub unsafe fn DefineScope(
        &self,
        rclsid: *const GUID,
        dwCreateFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Open scope from a file.
    pub unsafe fn OpenScope(
        &self,
        szScope: *const u16,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Open scope from memory.
    pub unsafe fn OpenScopeOnMemory(
        &self,
        pData: *const c_void,
        cbData: u32,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// IMetaDataDispenserEx - Extended metadata dispenser.
#[interface("31BCFCE2-DAFB-11D2-9F81-00C04F79A0A3")]
pub unsafe trait IMetaDataDispenserEx: IUnknown {
    // IMetaDataDispenser methods
    pub unsafe fn DefineScope(
        &self,
        rclsid: *const GUID,
        dwCreateFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    pub unsafe fn OpenScope(
        &self,
        szScope: *const u16,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    pub unsafe fn OpenScopeOnMemory(
        &self,
        pData: *const c_void,
        cbData: u32,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    // IMetaDataDispenserEx methods
    /// Set an option.
    pub unsafe fn SetOption(&self, optionid: *const GUID, value: *const c_void) -> HRESULT;

    /// Get an option.
    pub unsafe fn GetOption(&self, optionid: *const GUID, pvalue: *mut c_void) -> HRESULT;

    /// Open scope from ITypeInfo.
    pub unsafe fn OpenScopeOnITypeInfo(
        &self,
        pITI: *mut IUnknown,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get cached COR system directory.
    pub unsafe fn GetCORSystemDirectory(
        &self,
        szBuffer: *mut u16,
        cchBuffer: u32,
        pchBuffer: *mut u32,
    ) -> HRESULT;

    /// Find assembly.
    pub unsafe fn FindAssembly(
        &self,
        szAppBase: *const u16,
        szPrivateBin: *const u16,
        szGlobalBin: *const u16,
        szAssemblyName: *const u16,
        szName: *mut u16,
        cchName: u32,
        pcName: *mut u32,
    ) -> HRESULT;

    /// Find assembly module.
    pub unsafe fn FindAssemblyModule(
        &self,
        szAppBase: *const u16,
        szPrivateBin: *const u16,
        szGlobalBin: *const u16,
        szAssemblyName: *const u16,
        szModuleName: *const u16,
        szName: *mut u16,
        cchName: u32,
        pcName: *mut u32,
    ) -> HRESULT;
}

/// IMetaDataError - Reports metadata errors.
#[interface("B81FF171-20F3-11D2-8DCC-00A0C9B09C19")]
pub unsafe trait IMetaDataError: IUnknown {
    /// Called when a metadata error occurs.
    pub unsafe fn OnError(&self, hrError: HRESULT, token: u32) -> HRESULT;
}

/// IMapToken - Maps tokens during metadata emit.
#[interface("06A3EA8B-0225-11D1-BF72-00C04FC31E12")]
pub unsafe trait IMapToken: IUnknown {
    /// Map a token.
    pub unsafe fn Map(&self, tkImp: u32, tkEmit: u32) -> HRESULT;
}

/// IMetaDataTables - Provides access to raw metadata tables.
#[interface("D8F579AB-402D-4B8E-82D9-5D63B1065C68")]
pub unsafe trait IMetaDataTables: IUnknown {
    /// Get the string heap size.
    pub unsafe fn GetStringHeapSize(&self, pcbStrings: *mut u32) -> HRESULT;

    /// Get the blob heap size.
    pub unsafe fn GetBlobHeapSize(&self, pcbBlobs: *mut u32) -> HRESULT;

    /// Get the GUID heap size.
    pub unsafe fn GetGuidHeapSize(&self, pcbGuids: *mut u32) -> HRESULT;

    /// Get the user string heap size.
    pub unsafe fn GetUserStringHeapSize(&self, pcbBlobs: *mut u32) -> HRESULT;

    /// Get the number of tables.
    pub unsafe fn GetNumTables(&self, pcTables: *mut u32) -> HRESULT;

    /// Get a table's info.
    pub unsafe fn GetTableIndex(&self, token: u32, pixTbl: *mut u32) -> HRESULT;

    /// Get table info.
    pub unsafe fn GetTableInfo(
        &self,
        ixTbl: u32,
        pcbRow: *mut u32,
        pcRows: *mut u32,
        pcCols: *mut u32,
        piKey: *mut u32,
        ppName: *mut *const u8,
    ) -> HRESULT;
}
