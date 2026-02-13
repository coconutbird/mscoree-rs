//! Fusion/Assembly binding interfaces (legacy .NET Framework).
//!
//! These interfaces provide access to the Global Assembly Cache (GAC)
//! and assembly binding functionality.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, PCWSTR, interface};

/// Assembly cache install flags.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ASM_CACHE_FLAGS {
    /// The GAC cache.
    ASM_CACHE_ZAP = 0x1,
    /// The GAC.
    ASM_CACHE_GAC = 0x2,
    /// The download cache.
    ASM_CACHE_DOWNLOAD = 0x4,
    /// Root GAC (CLR 4.0+).
    ASM_CACHE_ROOT = 0x8,
    /// Root ex GAC.
    ASM_CACHE_ROOT_EX = 0x80,
}

/// Assembly name display flags.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ASM_DISPLAY_FLAGS {
    VERSION = 0x1,
    CULTURE = 0x2,
    PUBLIC_KEY_TOKEN = 0x4,
    PUBLIC_KEY = 0x8,
    CUSTOM = 0x10,
    PROCESSORARCHITECTURE = 0x20,
    LANGUAGEID = 0x40,
    RETARGET = 0x80,
    CONFIG_MASK = 0x100,
    MVID = 0x200,
    CONTENT_TYPE = 0x400,
}

/// Assembly name property enumeration.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ASM_NAME {
    PUBLIC_KEY = 0,
    PUBLIC_KEY_TOKEN = 1,
    HASH_VALUE = 2,
    NAME = 3,
    MAJOR_VERSION = 4,
    MINOR_VERSION = 5,
    BUILD_NUMBER = 6,
    REVISION_NUMBER = 7,
    CULTURE = 8,
    PROCESSOR_ID_ARRAY = 9,
    OSINFO_ARRAY = 10,
    HASH_ALGID = 11,
    ALIAS = 12,
    CODEBASE_URL = 13,
    CODEBASE_LASTMOD = 14,
    NULL_PUBLIC_KEY = 15,
    NULL_PUBLIC_KEY_TOKEN = 16,
    CUSTOM = 17,
    NULL_CUSTOM = 18,
    MVID = 19,
    FILE_MAJOR_VERSION = 20,
    FILE_MINOR_VERSION = 21,
    FILE_BUILD_NUMBER = 22,
    FILE_REVISION_NUMBER = 23,
    RETARGET = 24,
    SIGNATURE_BLOB = 25,
    CONFIG_MASK = 26,
    ARCHITECTURE = 27,
    CONTENT_TYPE = 28,
    MAX_PARAMS = 29,
}

/// ASSEMBLY_INFO structure for assembly cache queries.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: *mut u16,
    pub cchBuf: u32,
}

impl Default for ASSEMBLY_INFO {
    fn default() -> Self {
        Self {
            cbAssemblyInfo: std::mem::size_of::<ASSEMBLY_INFO>() as u32,
            dwAssemblyFlags: 0,
            uliAssemblySizeInKB: 0,
            pszCurrentAssemblyPathBuf: std::ptr::null_mut(),
            cchBuf: 0,
        }
    }
}

/// IAssemblyCache - Provides access to the Global Assembly Cache.
#[interface("E707DCDE-D1CD-11D2-BAB9-00C04F8ECEAE")]
pub unsafe trait IAssemblyCache: IUnknown {
    /// Uninstall an assembly from the cache.
    pub unsafe fn UninstallAssembly(
        &self,
        dwFlags: u32,
        pszAssemblyName: PCWSTR,
        pRefData: *const c_void,
        pulDisposition: *mut u32,
    ) -> HRESULT;

    /// Query assembly information.
    pub unsafe fn QueryAssemblyInfo(
        &self,
        dwFlags: u32,
        pszAssemblyName: PCWSTR,
        pAsmInfo: *mut ASSEMBLY_INFO,
    ) -> HRESULT;

    /// Create assembly cache item.
    pub unsafe fn CreateAssemblyCacheItem(
        &self,
        dwFlags: u32,
        pvReserved: *const c_void,
        ppAsmItem: *mut *mut IAssemblyCacheItem,
        pszAssemblyName: PCWSTR,
    ) -> HRESULT;

    /// Reserved (placeholder).
    pub unsafe fn Reserved(&self, ppUnk: *mut *mut IUnknown) -> HRESULT;

    /// Install an assembly into the cache.
    pub unsafe fn InstallAssembly(
        &self,
        dwFlags: u32,
        pszManifestFilePath: PCWSTR,
        pRefData: *const c_void,
    ) -> HRESULT;
}

/// IAssemblyCacheItem - Represents an item being installed in the cache.
#[interface("9E3AAEB4-D1CD-11D2-BAB9-00C04F8ECEAE")]
pub unsafe trait IAssemblyCacheItem: IUnknown {
    /// Create a stream for the assembly file.
    pub unsafe fn CreateStream(
        &self,
        dwFlags: u32,
        pszStreamName: PCWSTR,
        dwFormat: u32,
        dwFormatFlags: u32,
        ppIStream: *mut *mut IUnknown,
        puliMaxSize: *mut u64,
    ) -> HRESULT;

    /// Commit the assembly to the cache.
    pub unsafe fn Commit(&self, dwFlags: u32, pulDisposition: *mut u32) -> HRESULT;

    /// Abort the installation.
    pub unsafe fn AbortItem(&self) -> HRESULT;
}

/// IAssemblyName - Represents an assembly's unique identity.
#[interface("CD193BC0-B4BC-11D2-9833-00C04FC31D2E")]
pub unsafe trait IAssemblyName: IUnknown {
    /// Set a property value.
    pub unsafe fn SetProperty(
        &self,
        PropertyId: u32,
        pvProperty: *const c_void,
        cbProperty: u32,
    ) -> HRESULT;

    /// Get a property value.
    pub unsafe fn GetProperty(
        &self,
        PropertyId: u32,
        pvProperty: *mut c_void,
        pcbProperty: *mut u32,
    ) -> HRESULT;

    /// Finalize the assembly name.
    pub unsafe fn Finalize(&self) -> HRESULT;

    /// Get the display name.
    pub unsafe fn GetDisplayName(
        &self,
        szDisplayName: *mut u16,
        pccDisplayName: *mut u32,
        dwDisplayFlags: u32,
    ) -> HRESULT;

    /// Reserved (binding).
    pub unsafe fn Reserved(
        &self,
        refIID: *const GUID,
        pUnkReserved1: *mut IUnknown,
        pUnkReserved2: *mut IUnknown,
        szReserved: PCWSTR,
        llReserved: i64,
        pvReserved: *mut c_void,
        cbReserved: u32,
        ppReserved: *mut *mut c_void,
    ) -> HRESULT;

    /// Get the assembly name.
    pub unsafe fn GetName(&self, lpcwBuffer: *mut u32, pwzName: *mut u16) -> HRESULT;

    /// Get the assembly version.
    pub unsafe fn GetVersion(&self, pdwVersionHi: *mut u32, pdwVersionLow: *mut u32) -> HRESULT;

    /// Check if two names are equal.
    pub unsafe fn IsEqual(&self, pName: *mut IAssemblyName, dwCmpFlags: u32) -> HRESULT;

    /// Clone this assembly name.
    pub unsafe fn Clone(&self, pName: *mut *mut IAssemblyName) -> HRESULT;
}

/// IAssemblyEnum - Enumerates assemblies in the cache.
#[interface("21B8916C-F28E-11D2-A473-00C04F8EF448")]
pub unsafe trait IAssemblyEnum: IUnknown {
    /// Get the next assembly in the enumeration.
    pub unsafe fn GetNextAssembly(
        &self,
        pvReserved: *mut c_void,
        ppName: *mut *mut IAssemblyName,
        dwFlags: u32,
    ) -> HRESULT;

    /// Reset the enumeration.
    pub unsafe fn Reset(&self) -> HRESULT;

    /// Clone the enumerator.
    pub unsafe fn Clone(&self, ppEnum: *mut *mut IAssemblyEnum) -> HRESULT;
}

/// IInstallReferenceItem - Represents an install reference.
#[interface("582DAC66-E678-449F-ABA6-6FAAEC8A9394")]
pub unsafe trait IInstallReferenceItem: IUnknown {
    /// Get the reference info.
    pub unsafe fn GetReference(
        &self,
        ppRefData: *mut *const c_void,
        dwFlags: u32,
        pvReserved: *mut c_void,
    ) -> HRESULT;
}

/// IInstallReferenceEnum - Enumerates install references.
#[interface("56B1A988-7C0C-4AA2-8639-C3EB5A90226F")]
pub unsafe trait IInstallReferenceEnum: IUnknown {
    /// Get the next reference.
    pub unsafe fn GetNextInstallReferenceItem(
        &self,
        ppRefItem: *mut *mut IInstallReferenceItem,
        dwFlags: u32,
        pvReserved: *mut c_void,
    ) -> HRESULT;
}
