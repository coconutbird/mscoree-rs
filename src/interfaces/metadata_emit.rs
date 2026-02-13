//! IMetaDataEmit interface definitions for writing assembly metadata.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IMetaDataEmit - Write metadata to an assembly.
#[interface("BA3FEE4C-ECB9-4E41-83B7-183FA41CD859")]
pub unsafe trait IMetaDataEmit: IUnknown {
    /// Set the module properties.
    pub unsafe fn SetModuleProps(&self, szName: *const u16) -> HRESULT;

    /// Save the metadata to a file.
    pub unsafe fn Save(&self, szFile: *const u16, dwSaveFlags: u32) -> HRESULT;

    /// Save the metadata to a stream.
    pub unsafe fn SaveToStream(&self, pIStream: *mut IUnknown, dwSaveFlags: u32) -> HRESULT;

    /// Get the save size.
    pub unsafe fn GetSaveSize(&self, fSave: u32, pdwSaveSize: *mut u32) -> HRESULT;

    /// Define a type definition.
    pub unsafe fn DefineTypeDef(
        &self,
        szTypeDef: *const u16,
        dwTypeDefFlags: u32,
        tkExtends: u32,
        rtkImplements: *const u32,
        ptd: *mut u32,
    ) -> HRESULT;

    /// Define a nested type.
    pub unsafe fn DefineNestedType(
        &self,
        szTypeDef: *const u16,
        dwTypeDefFlags: u32,
        tkExtends: u32,
        rtkImplements: *const u32,
        tdEncloser: u32,
        ptd: *mut u32,
    ) -> HRESULT;

    /// Set the handler for token remapping.
    pub unsafe fn SetHandler(&self, pUnk: *mut IUnknown) -> HRESULT;

    /// Define a method.
    pub unsafe fn DefineMethod(
        &self,
        td: u32,
        szName: *const u16,
        dwMethodFlags: u32,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        ulCodeRVA: u32,
        dwImplFlags: u32,
        pmd: *mut u32,
    ) -> HRESULT;

    /// Define a method impl.
    pub unsafe fn DefineMethodImpl(&self, td: u32, tkBody: u32, tkDecl: u32) -> HRESULT;

    /// Define a type reference.
    pub unsafe fn DefineTypeRefByName(
        &self,
        tkResolutionScope: u32,
        szName: *const u16,
        ptr: *mut u32,
    ) -> HRESULT;

    /// Define an import type.
    pub unsafe fn DefineImportType(
        &self,
        pAssemImport: *mut IUnknown,
        pbHashValue: *const c_void,
        cbHashValue: u32,
        pImport: *mut IUnknown,
        tdImport: u32,
        pAssemEmit: *mut IUnknown,
        ptr: *mut u32,
    ) -> HRESULT;

    /// Define a member reference.
    pub unsafe fn DefineMemberRef(
        &self,
        tkImport: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmr: *mut u32,
    ) -> HRESULT;

    /// Define an import member.
    pub unsafe fn DefineImportMember(
        &self,
        pAssemImport: *mut IUnknown,
        pbHashValue: *const c_void,
        cbHashValue: u32,
        pImport: *mut IUnknown,
        mbMember: u32,
        pAssemEmit: *mut IUnknown,
        tkParent: u32,
        pmr: *mut u32,
    ) -> HRESULT;

    /// Define an event.
    pub unsafe fn DefineEvent(
        &self,
        td: u32,
        szEvent: *const u16,
        dwEventFlags: u32,
        tkEventType: u32,
        mdAddOn: u32,
        mdRemoveOn: u32,
        mdFire: u32,
        rmdOtherMethods: *const u32,
        pmdEvent: *mut u32,
    ) -> HRESULT;

    /// Set the class layout.
    pub unsafe fn SetClassLayout(
        &self,
        td: u32,
        dwPackSize: u32,
        rFieldOffsets: *const c_void,
        ulClassSize: u32,
    ) -> HRESULT;

    /// Delete class layout.
    pub unsafe fn DeleteClassLayout(&self, td: u32) -> HRESULT;

    /// Set the field marshal.
    pub unsafe fn SetFieldMarshal(
        &self,
        tk: u32,
        pvNativeType: *const u8,
        cbNativeType: u32,
    ) -> HRESULT;

    /// Delete field marshal.
    pub unsafe fn DeleteFieldMarshal(&self, tk: u32) -> HRESULT;

    /// Define a permission set.
    pub unsafe fn DefinePermissionSet(
        &self,
        tk: u32,
        dwAction: u32,
        pvPermission: *const c_void,
        cbPermission: u32,
        ppm: *mut u32,
    ) -> HRESULT;

    /// Set the RVA.
    pub unsafe fn SetRVA(&self, md: u32, ulRVA: u32) -> HRESULT;

    /// Get a token from a type spec.
    pub unsafe fn GetTokenFromTypeSpec(
        &self,
        pvSig: *const u8,
        cbSig: u32,
        ptypespec: *mut u32,
    ) -> HRESULT;

    /// Save to memory.
    pub unsafe fn SaveToMemory(&self, pbData: *mut c_void, cbData: u32) -> HRESULT;
}
