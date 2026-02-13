//! IMetaDataAssemblyImport and IMetaDataAssemblyEmit interface definitions.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IMetaDataAssemblyImport - Read assembly-level metadata.
#[interface("EE62470B-E94B-424E-9B7C-2F00C9249F93")]
pub unsafe trait IMetaDataAssemblyImport: IUnknown {
    /// Get the assembly from the scope.
    pub unsafe fn GetAssemblyFromScope(&self, ptkAssembly: *mut u32) -> HRESULT;

    /// Get assembly properties.
    pub unsafe fn GetAssemblyProps(
        &self,
        mda: u32,
        ppbPublicKey: *mut *const c_void,
        pcbPublicKey: *mut u32,
        pulHashAlgId: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pMetaData: *mut c_void, // ASSEMBLYMETADATA*
        pdwAssemblyFlags: *mut u32,
    ) -> HRESULT;

    /// Get assembly ref properties.
    pub unsafe fn GetAssemblyRefProps(
        &self,
        mdar: u32,
        ppbPublicKeyOrToken: *mut *const c_void,
        pcbPublicKeyOrToken: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pMetaData: *mut c_void, // ASSEMBLYMETADATA*
        ppbHashValue: *mut *const c_void,
        pcbHashValue: *mut u32,
        pdwAssemblyRefFlags: *mut u32,
    ) -> HRESULT;

    /// Get file properties.
    pub unsafe fn GetFileProps(
        &self,
        mdf: u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        ppbHashValue: *mut *const c_void,
        pcbHashValue: *mut u32,
        pdwFileFlags: *mut u32,
    ) -> HRESULT;

    /// Get exported type properties.
    pub unsafe fn GetExportedTypeProps(
        &self,
        mdct: u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        ptkImplementation: *mut u32,
        ptkTypeDef: *mut u32,
        pdwExportedTypeFlags: *mut u32,
    ) -> HRESULT;

    /// Get manifest resource properties.
    pub unsafe fn GetManifestResourceProps(
        &self,
        mdmr: u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        ptkImplementation: *mut u32,
        pdwOffset: *mut u32,
        pdwResourceFlags: *mut u32,
    ) -> HRESULT;

    /// Enumerate assembly references.
    pub unsafe fn EnumAssemblyRefs(
        &self,
        phEnum: *mut *mut c_void,
        rAssemblyRefs: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate files.
    pub unsafe fn EnumFiles(
        &self,
        phEnum: *mut *mut c_void,
        rFiles: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate exported types.
    pub unsafe fn EnumExportedTypes(
        &self,
        phEnum: *mut *mut c_void,
        rExportedTypes: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate manifest resources.
    pub unsafe fn EnumManifestResources(
        &self,
        phEnum: *mut *mut c_void,
        rManifestResources: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Close an enumeration.
    pub unsafe fn CloseEnum(&self, hEnum: *mut c_void) -> HRESULT;

    /// Find exported type by name.
    pub unsafe fn FindExportedTypeByName(
        &self,
        szName: *const u16,
        mdtExportedType: u32,
        ptkExportedType: *mut u32,
    ) -> HRESULT;

    /// Find manifest resource by name.
    pub unsafe fn FindManifestResourceByName(
        &self,
        szName: *const u16,
        ptkManifestResource: *mut u32,
    ) -> HRESULT;

    /// Find assemblies by name.
    pub unsafe fn FindAssembliesByName(
        &self,
        szAppBase: *const u16,
        szPrivateBin: *const u16,
        szAssemblyName: *const u16,
        ppIUnk: *mut *mut IUnknown,
        cMax: u32,
        pcAssemblies: *mut u32,
    ) -> HRESULT;
}

/// IMetaDataAssemblyEmit - Write assembly-level metadata.
#[interface("211EF15B-5317-4438-B196-DEC87B887693")]
pub unsafe trait IMetaDataAssemblyEmit: IUnknown {
    /// Define an assembly.
    pub unsafe fn DefineAssembly(
        &self,
        pbPublicKey: *const c_void,
        cbPublicKey: u32,
        ulHashAlgId: u32,
        szName: *const u16,
        pMetaData: *const c_void, // ASSEMBLYMETADATA*
        dwAssemblyFlags: u32,
        pmda: *mut u32,
    ) -> HRESULT;

    /// Define an assembly reference.
    pub unsafe fn DefineAssemblyRef(
        &self,
        pbPublicKeyOrToken: *const c_void,
        cbPublicKeyOrToken: u32,
        szName: *const u16,
        pMetaData: *const c_void, // ASSEMBLYMETADATA*
        pbHashValue: *const c_void,
        cbHashValue: u32,
        dwAssemblyRefFlags: u32,
        pmdar: *mut u32,
    ) -> HRESULT;
}
