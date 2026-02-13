//! IXCLRDataProcess interface definitions for CLR debugging.
//!
//! These interfaces provide methods for querying information about a CLR process.
//! They are used by debuggers and data access services.
//!
//! Available since .NET Framework 4.7.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::data_target::CLRDATA_ADDRESS;

/// Opaque handle type for CLR data enumerations.
pub type CLRDATA_ENUM = u64;

/// Buffer for follow-stub operations.
#[repr(C)]
pub struct CLRDATA_FOLLOW_STUB_BUFFER {
    pub Data: [u64; 8],
}

/// Address range structure.
#[repr(C)]
pub struct CLRDATA_ADDRESS_RANGE {
    pub startAddress: CLRDATA_ADDRESS,
    pub endAddress: CLRDATA_ADDRESS,
}

/// Exception record structure (64-bit).
#[repr(C)]
pub struct EXCEPTION_RECORD64 {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u64,
    pub ExceptionAddress: u64,
    pub NumberParameters: u32,
    pub __unusedAlignment: u32,
    pub ExceptionInformation: [u64; 15], // EXCEPTION_MAXIMUM_PARAMETERS
}

/// CLR data address type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataAddressType {
    CLRDATA_ADDRESS_UNRECOGNIZED = 0,
    CLRDATA_ADDRESS_MANAGED_METHOD = 1,
    CLRDATA_ADDRESS_RUNTIME_MANAGED_CODE = 2,
    CLRDATA_ADDRESS_RUNTIME_UNMANAGED_CODE = 3,
    CLRDATA_ADDRESS_GC_DATA = 4,
    CLRDATA_ADDRESS_RUNTIME_MANAGED_STUB = 5,
    CLRDATA_ADDRESS_RUNTIME_UNMANAGED_STUB = 6,
}

// Forward declarations for related interfaces (defined as opaque IUnknown for now)
// These can be implemented as full interfaces in separate files as needed.

/// IXCLRDataTask - Represents a managed task (thread).
#[interface("A5B0BEEA-EC62-4618-8012-A24FFC23934C")]
pub unsafe trait IXCLRDataTask: IUnknown {}

/// IXCLRDataAppDomain - Represents an application domain.
#[interface("7CA04601-C702-4670-A63C-FA44F7DA7BD5")]
pub unsafe trait IXCLRDataAppDomain: IUnknown {}

/// IXCLRDataAssembly - Represents an assembly.
#[interface("2FA17588-43C2-46AB-9B51-C8F01E39C9AC")]
pub unsafe trait IXCLRDataAssembly: IUnknown {}

/// IXCLRDataModule - Represents a managed module.
#[interface("88E32849-0A0A-4CB0-9022-7CD2E9E139E2")]
pub unsafe trait IXCLRDataModule: IUnknown {}

/// IXCLRDataMethodInstance - Represents a method instance.
#[interface("ECD73800-22CA-4B0D-AB55-E9BA7E6318A5")]
pub unsafe trait IXCLRDataMethodInstance: IUnknown {}

/// IXCLRDataMethodDefinition - Represents a method definition.
#[interface("AAF60008-FB2C-420B-8FB1-42D244A54A97")]
pub unsafe trait IXCLRDataMethodDefinition: IUnknown {}

/// IXCLRDataValue - Represents a CLR value.
#[interface("96EC93C7-1000-4E93-8991-98D8766E6666")]
pub unsafe trait IXCLRDataValue: IUnknown {}

/// IXCLRDataExceptionState - Represents exception state.
#[interface("75DA9E4C-BD33-43C8-8F5C-96E8A5241F57")]
pub unsafe trait IXCLRDataExceptionState: IUnknown {}

/// IXCLRDataExceptionNotification - Exception notification callback.
#[interface("2D95A079-42A1-4837-818F-0B97D7048E0E")]
pub unsafe trait IXCLRDataExceptionNotification: IUnknown {}

/// IXCLRDataTypeInstance - Represents a type instance.
#[interface("4D078D91-B8FF-4D18-AEEE-F24C881F7384")]
pub unsafe trait IXCLRDataTypeInstance: IUnknown {}

/// IXCLRDataProcess interface for CLR debugging (.NET 4.7+).
///
/// Provides methods for querying information about a managed process.
/// This interface is obtained through the CLRDataCreateInstance function
/// exported by mscordacwks.dll.
#[interface("5C552AB6-FC09-4CB3-8E36-22FA03C798B7")]
pub unsafe trait IXCLRDataProcess: IUnknown {
    /// Flush any cached data for this process.
    /// All IXCLR* interfaces obtained for this process will become invalid.
    pub unsafe fn Flush(&self) -> HRESULT;

    /// Begin enumeration of tasks (threads).
    /// Returns S_FALSE if the enumeration is empty.
    pub unsafe fn StartEnumTasks(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next task in the enumeration.
    /// Returns S_FALSE if there isn't a next entry.
    pub unsafe fn EnumTask(
        &self,
        handle: *mut CLRDATA_ENUM,
        task: *mut *mut IXCLRDataTask,
    ) -> HRESULT;

    /// Release the task enumerator.
    pub unsafe fn EndEnumTasks(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get the managed task running on the given OS thread ID.
    pub unsafe fn GetTaskByOSThreadID(
        &self,
        osThreadID: u32,
        task: *mut *mut IXCLRDataTask,
    ) -> HRESULT;

    /// Get the managed task corresponding to the given task ID.
    pub unsafe fn GetTaskByUniqueID(
        &self,
        taskID: u64,
        task: *mut *mut IXCLRDataTask,
    ) -> HRESULT;

    /// Get state flags (see CLRDataProcessFlag).
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether the given interface represents the same target state.
    pub unsafe fn IsSameObject(&self, process: *mut IXCLRDataProcess) -> HRESULT;

    /// Get the managed object representing the process.
    pub unsafe fn GetManagedObject(&self, value: *mut *mut IXCLRDataValue) -> HRESULT;

    /// Get the desired execution state.
    pub unsafe fn GetDesiredExecutionState(&self, state: *mut u32) -> HRESULT;

    /// Set the desired execution state.
    pub unsafe fn SetDesiredExecutionState(&self, state: u32) -> HRESULT;

    /// Return an indicator of the type of data referred to by the given address.
    pub unsafe fn GetAddressType(
        &self,
        address: CLRDATA_ADDRESS,
        r#type: *mut CLRDataAddressType,
    ) -> HRESULT;

    /// Get a name for the given address if it refers to non-managed-method info.
    /// Returns S_FALSE if the buffer is not large enough for the name.
    pub unsafe fn GetRuntimeNameByAddress(
        &self,
        address: CLRDATA_ADDRESS,
        flags: u32,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
        displacement: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Begin enumeration of app domains.
    pub unsafe fn StartEnumAppDomains(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next app domain in the enumeration.
    pub unsafe fn EnumAppDomain(
        &self,
        handle: *mut CLRDATA_ENUM,
        appDomain: *mut *mut IXCLRDataAppDomain,
    ) -> HRESULT;

    /// Release the app domain enumerator.
    pub unsafe fn EndEnumAppDomains(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Find an app domain by its unique ID.
    pub unsafe fn GetAppDomainByUniqueID(
        &self,
        id: u64,
        appDomain: *mut *mut IXCLRDataAppDomain,
    ) -> HRESULT;

    /// Begin enumeration of assemblies.
    pub unsafe fn StartEnumAssemblies(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next assembly in the enumeration.
    pub unsafe fn EnumAssembly(
        &self,
        handle: *mut CLRDATA_ENUM,
        assembly: *mut *mut IXCLRDataAssembly,
    ) -> HRESULT;

    /// Release the assembly enumerator.
    pub unsafe fn EndEnumAssemblies(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Begin enumeration of modules.
    pub unsafe fn StartEnumModules(&self, handle: *mut CLRDATA_ENUM) -> HRESULT;

    /// Get the next module in the enumeration.
    pub unsafe fn EnumModule(
        &self,
        handle: *mut CLRDATA_ENUM,
        module: *mut *mut IXCLRDataModule,
    ) -> HRESULT;

    /// Release the module enumerator.
    pub unsafe fn EndEnumModules(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Look up a module by address.
    pub unsafe fn GetModuleByAddress(
        &self,
        address: CLRDATA_ADDRESS,
        module: *mut *mut IXCLRDataModule,
    ) -> HRESULT;

    /// Begin enumeration of method instances by native code address.
    pub unsafe fn StartEnumMethodInstancesByAddress(
        &self,
        address: CLRDATA_ADDRESS,
        appDomain: *mut IXCLRDataAppDomain,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next method instance in the enumeration.
    pub unsafe fn EnumMethodInstanceByAddress(
        &self,
        handle: *mut CLRDATA_ENUM,
        method: *mut *mut IXCLRDataMethodInstance,
    ) -> HRESULT;

    /// Release the method instance enumerator.
    pub unsafe fn EndEnumMethodInstancesByAddress(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Look up the name and value of data by address.
    pub unsafe fn GetDataByAddress(
        &self,
        address: CLRDATA_ADDRESS,
        flags: u32,
        appDomain: *mut IXCLRDataAppDomain,
        tlsTask: *mut IXCLRDataTask,
        bufLen: u32,
        nameLen: *mut u32,
        nameBuf: *mut u16,
        value: *mut *mut IXCLRDataValue,
        displacement: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Get managed exception state for a system exception (OBSOLETE).
    pub unsafe fn GetExceptionStateByExceptionRecord(
        &self,
        record: *const EXCEPTION_RECORD64,
        exState: *mut *mut IXCLRDataExceptionState,
    ) -> HRESULT;

    /// Translate a system exception record into a notification.
    pub unsafe fn TranslateExceptionRecordToNotification(
        &self,
        record: *const EXCEPTION_RECORD64,
        notify: *mut IXCLRDataExceptionNotification,
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

    /// Create a simple value based on type and address.
    pub unsafe fn CreateMemoryValue(
        &self,
        appDomain: *mut IXCLRDataAppDomain,
        tlsTask: *mut IXCLRDataTask,
        r#type: *mut IXCLRDataTypeInstance,
        addr: CLRDATA_ADDRESS,
        value: *mut *mut IXCLRDataValue,
    ) -> HRESULT;

    /// Update all type notifications for a module.
    pub unsafe fn SetAllTypeNotifications(
        &self,
        module: *mut IXCLRDataModule,
        flags: u32,
    ) -> HRESULT;

    /// Update all code notifications for a module.
    pub unsafe fn SetAllCodeNotifications(
        &self,
        module: *mut IXCLRDataModule,
        flags: u32,
    ) -> HRESULT;

    /// Get type notifications for tokens.
    pub unsafe fn GetTypeNotifications(
        &self,
        numTokens: u32,
        mods: *mut *mut IXCLRDataModule,
        singleMod: *mut IXCLRDataModule,
        tokens: *const u32,
        flags: *mut u32,
    ) -> HRESULT;

    /// Set type notifications for tokens.
    pub unsafe fn SetTypeNotifications(
        &self,
        numTokens: u32,
        mods: *mut *mut IXCLRDataModule,
        singleMod: *mut IXCLRDataModule,
        tokens: *const u32,
        flags: *const u32,
        singleFlags: u32,
    ) -> HRESULT;

    /// Get code notifications for tokens.
    pub unsafe fn GetCodeNotifications(
        &self,
        numTokens: u32,
        mods: *mut *mut IXCLRDataModule,
        singleMod: *mut IXCLRDataModule,
        tokens: *const u32,
        flags: *mut u32,
    ) -> HRESULT;

    /// Set code notifications for tokens.
    pub unsafe fn SetCodeNotifications(
        &self,
        numTokens: u32,
        mods: *mut *mut IXCLRDataModule,
        singleMod: *mut IXCLRDataModule,
        tokens: *const u32,
        flags: *const u32,
        singleFlags: u32,
    ) -> HRESULT;

    /// Get other notification flags.
    pub unsafe fn GetOtherNotificationFlags(&self, flags: *mut u32) -> HRESULT;

    /// Set other notification flags.
    pub unsafe fn SetOtherNotificationFlags(&self, flags: u32) -> HRESULT;

    /// Begin enumeration of method definitions by IL code address.
    pub unsafe fn StartEnumMethodDefinitionsByAddress(
        &self,
        address: CLRDATA_ADDRESS,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next method definition in the enumeration.
    pub unsafe fn EnumMethodDefinitionByAddress(
        &self,
        handle: *mut CLRDATA_ENUM,
        method: *mut *mut IXCLRDataMethodDefinition,
    ) -> HRESULT;

    /// Release the method definition enumerator.
    pub unsafe fn EndEnumMethodDefinitionsByAddress(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Follow a CLR stub to determine next execution address (OBSOLETE: Use FollowStub2).
    pub unsafe fn FollowStub(
        &self,
        inFlags: u32,
        inAddr: CLRDATA_ADDRESS,
        inBuffer: *const CLRDATA_FOLLOW_STUB_BUFFER,
        outAddr: *mut CLRDATA_ADDRESS,
        outBuffer: *mut CLRDATA_FOLLOW_STUB_BUFFER,
        outFlags: *mut u32,
    ) -> HRESULT;

    /// Follow a CLR stub with task context (requires revision 7).
    pub unsafe fn FollowStub2(
        &self,
        task: *mut IXCLRDataTask,
        inFlags: u32,
        inAddr: CLRDATA_ADDRESS,
        inBuffer: *const CLRDATA_FOLLOW_STUB_BUFFER,
        outAddr: *mut CLRDATA_ADDRESS,
        outBuffer: *mut CLRDATA_FOLLOW_STUB_BUFFER,
        outFlags: *mut u32,
    ) -> HRESULT;

    // DumpNativeImage is omitted as it requires IXCLRDataDisplay and related interfaces
}

