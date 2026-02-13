//! IXCLRDataModule interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::xclr_data_process::CLRDATA_ENUM;
use super::xclr_data_types::CLRDATA_METHDEF_EXTENT;

/// IXCLRDataModule - Represents a managed module.
#[interface("88E32849-0A0A-4CB0-9022-7CD2E9E139E2")]
pub unsafe trait IXCLRDataModule: IUnknown {
    /// Begin enumeration of assemblies containing this module.
    pub unsafe fn StartEnumAssemblies(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next assembly in the enumeration.
    pub unsafe fn EnumAssembly(
        &self,
        handle: *mut CLRDATA_ENUM,
        assembly: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of assemblies.
    pub unsafe fn EndEnumAssemblies(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of type definitions.
    pub unsafe fn StartEnumTypeDefinitions(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next type definition in the enumeration.
    pub unsafe fn EnumTypeDefinition(
        &self,
        handle: *mut CLRDATA_ENUM,
        typeDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of type definitions.
    pub unsafe fn EndEnumTypeDefinitions(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of type instances.
    pub unsafe fn StartEnumTypeInstances(
        &self,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next type instance in the enumeration.
    pub unsafe fn EnumTypeInstance(
        &self,
        handle: *mut CLRDATA_ENUM,
        typeInstance: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of type instances.
    pub unsafe fn EndEnumTypeInstances(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get a type definition by its metadata token.
    pub unsafe fn GetTypeDefinitionByToken(
        &self,
        token: u32,
        typeDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Begin enumeration of type definitions by name.
    pub unsafe fn StartEnumTypeDefinitionsByName(
        &self,
        flags: u32,
        typeCount: u32,
        names: *const *const u16,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next type definition by name.
    pub unsafe fn EnumTypeDefinitionByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        typeDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of type definitions by name.
    pub unsafe fn EndEnumTypeDefinitionsByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of type instances by name.
    pub unsafe fn StartEnumTypeInstancesByName(
        &self,
        flags: u32,
        typeCount: u32,
        names: *const *const u16,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next type instance by name.
    pub unsafe fn EnumTypeInstanceByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        typeInstance: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of type instances by name.
    pub unsafe fn EndEnumTypeInstancesByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get a method definition by its metadata token.
    pub unsafe fn GetMethodDefinitionByToken(
        &self,
        token: u32,
        methodDefinition: *mut *mut IUnknown,
    ) -> HRESULT;

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

    /// Begin enumeration of method instances by name.
    pub unsafe fn StartEnumMethodInstancesByName(
        &self,
        flags: u32,
        name: *const u16,
        appDomain: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next method instance by name.
    pub unsafe fn EnumMethodInstanceByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        methodInstance: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of method instances by name.
    pub unsafe fn EndEnumMethodInstancesByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get a static field data by name.
    pub unsafe fn StartEnumDataByName(
        &self,
        flags: u32,
        name: *const u16,
        appDomain: *mut IUnknown,
        tlsTask: *mut IUnknown,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next data by name.
    pub unsafe fn EnumDataByName(
        &self,
        handle: *mut CLRDATA_ENUM,
        value: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of data by name.
    pub unsafe fn EndEnumDataByName(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the name of this module.
    pub unsafe fn GetName(&self, bufLen: u32, nameLen: *mut u32, name: *mut u16) -> HRESULT;

    /// Get the file name of this module.
    pub unsafe fn GetFileName(&self, bufLen: u32, nameLen: *mut u32, name: *mut u16) -> HRESULT;

    /// Get the flags for this module.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether this is the same object as another.
    pub unsafe fn IsSameObject(&self, module: *mut IXCLRDataModule) -> HRESULT;

    /// Begin enumeration of code extents for all methods.
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

    /// Begin enumeration of app domains containing this module.
    pub unsafe fn StartEnumAppDomains(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next app domain.
    pub unsafe fn EnumAppDomain(
        &self,
        handle: *mut CLRDATA_ENUM,
        appDomain: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of app domains.
    pub unsafe fn EndEnumAppDomains(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get version ID of this module.
    pub unsafe fn GetVersionId(&self, vid: *mut GUID) -> HRESULT;
}
