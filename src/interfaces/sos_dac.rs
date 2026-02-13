//! ISOSDacInterface definitions for SOS debugging.
//!
//! These interfaces provide methods for SOS (Son of Strike) debugger extension
//! to query the CLR runtime for internal data.
//!
//! Available since .NET Framework 4.0.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::clr_data_target::CLRDATA_ADDRESS;

// Forward declaration for IXCLRDataModule
#[interface("88E32849-0A0A-4CB0-9022-7CD2E9E139E2")]
pub unsafe trait IXCLRDataModule_SOS: IUnknown {}

/// Handle data for GC handles.
#[repr(C)]
pub struct SOSHandleData {
    pub AppDomain: CLRDATA_ADDRESS,
    pub Handle: CLRDATA_ADDRESS,
    pub Secondary: CLRDATA_ADDRESS,
    pub Type: u32,
    pub StrongReference: i32,
    pub RefCount: u32,
    pub JupiterRefCount: u32,
    pub IsPegged: i32,
}

/// Stack source type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SOSStackSourceType {
    SOS_StackSourceIP = 0,
    SOS_StackSourceFrame = 1,
}

/// Stack reference flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SOSRefFlags {
    SOSRefInterior = 1,
    SOSRefPinned = 2,
}

/// Stack reference data.
#[repr(C)]
pub struct SOSStackRefData {
    pub HasRegisterInformation: i32,
    pub Register: i32,
    pub Offset: i32,
    pub Address: CLRDATA_ADDRESS,
    pub Object: CLRDATA_ADDRESS,
    pub Flags: u32,
    pub SourceType: SOSStackSourceType,
    pub Source: CLRDATA_ADDRESS,
    pub StackPointer: CLRDATA_ADDRESS,
}

/// Stack reference error data.
#[repr(C)]
pub struct SOSStackRefError {
    pub SourceType: SOSStackSourceType,
    pub Source: CLRDATA_ADDRESS,
    pub StackPointer: CLRDATA_ADDRESS,
}

/// Memory region data.
#[repr(C)]
pub struct SOSMemoryRegion {
    pub Start: CLRDATA_ADDRESS,
    pub Size: CLRDATA_ADDRESS,
    pub ExtraData: CLRDATA_ADDRESS,
    pub Heap: i32,
}

/// Method data for method enumeration.
#[repr(C)]
pub struct SOSMethodData {
    pub MethodDesc: CLRDATA_ADDRESS,
    pub Entrypoint: CLRDATA_ADDRESS,
    pub DefiningMethodTable: CLRDATA_ADDRESS,
    pub DefiningModule: CLRDATA_ADDRESS,
    pub Token: u32,
    pub Slot: u32,
}

/// ISOSEnum - Base enumeration interface.
#[interface("286CA186-E763-4F61-9760-487D43AE4341")]
pub unsafe trait ISOSEnum: IUnknown {
    pub unsafe fn Skip(&self, count: u32) -> HRESULT;
    pub unsafe fn Reset(&self) -> HRESULT;
    pub unsafe fn GetCount(&self, pCount: *mut u32) -> HRESULT;
}

/// ISOSHandleEnum - Handle enumeration interface.
#[interface("3E269830-4A2B-4301-8EE2-D6805B29B2FA")]
pub unsafe trait ISOSHandleEnum: ISOSEnum {
    pub unsafe fn Next(
        &self,
        count: u32,
        handles: *mut SOSHandleData,
        pNeeded: *mut u32,
    ) -> HRESULT;
}

/// ISOSStackRefErrorEnum - Stack reference error enumeration.
#[interface("774F4E1B-FB7B-491B-976D-A8130FE355E9")]
pub unsafe trait ISOSStackRefErrorEnum: ISOSEnum {
    pub unsafe fn Next(
        &self,
        count: u32,
        refs: *mut SOSStackRefError,
        pFetched: *mut u32,
    ) -> HRESULT;
}

/// ISOSStackRefEnum - Stack reference enumeration.
#[interface("8FA642BD-9F10-4799-9AA3-512AE78C77EE")]
pub unsafe trait ISOSStackRefEnum: ISOSEnum {
    pub unsafe fn Next(
        &self,
        count: u32,
        refs: *mut SOSStackRefData,
        pFetched: *mut u32,
    ) -> HRESULT;

    pub unsafe fn EnumerateErrors(&self, ppEnum: *mut *mut ISOSStackRefErrorEnum) -> HRESULT;
}

/// ISOSMemoryEnum - Memory region enumeration.
#[interface("E4B860EC-337A-40C0-A591-F09A9680690F")]
pub unsafe trait ISOSMemoryEnum: ISOSEnum {
    pub unsafe fn Next(
        &self,
        count: u32,
        memRegion: *mut SOSMemoryRegion,
        pNeeded: *mut u32,
    ) -> HRESULT;
}

/// ISOSMethodEnum - Method enumeration.
#[interface("3C0FE725-C324-4A4F-8100-D399588A662E")]
pub unsafe trait ISOSMethodEnum: ISOSEnum {
    pub unsafe fn Next(
        &self,
        count: u32,
        methods: *mut SOSMethodData,
        pNeeded: *mut u32,
    ) -> HRESULT;
}

// ============================================================================
// DAC Data Structures
// These structures are used by the SOS debugging interfaces to return data
// from the CLR runtime. They have fixed layouts for backwards compatibility.
// ============================================================================

/// Number of GC generations (0, 1, 2, LOH)
pub const DAC_NUMBERGENERATIONS: usize = 4;

/// Thread store data - information about all managed threads.
/// Size: 0x38 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpThreadStoreData {
    pub threadCount: i32,
    pub unstartedThreadCount: i32,
    pub backgroundThreadCount: i32,
    pub pendingThreadCount: i32,
    pub deadThreadCount: i32,
    pub firstThread: CLRDATA_ADDRESS,
    pub finalizerThread: CLRDATA_ADDRESS,
    pub gcThread: CLRDATA_ADDRESS,
    pub fHostConfig: u32,
}

/// AppDomain store data - information about all AppDomains.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpAppDomainStoreData {
    pub sharedDomain: CLRDATA_ADDRESS,
    pub systemDomain: CLRDATA_ADDRESS,
    pub DomainCount: i32,
}

/// AppDomain data stage enumeration.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DacpAppDomainDataStage {
    #[default]
    STAGE_CREATING = 0,
    STAGE_READYFORMANAGEDCODE = 1,
    STAGE_ACTIVE = 2,
    STAGE_OPEN = 3,
    STAGE_UNLOAD_REQUESTED = 4,
    STAGE_EXITING = 5,
    STAGE_EXITED = 6,
    STAGE_FINALIZING = 7,
    STAGE_FINALIZED = 8,
    STAGE_HANDLETABLE_NOACCESS = 9,
    STAGE_CLEARED = 10,
    STAGE_COLLECTED = 11,
    STAGE_CLOSED = 12,
}

/// AppDomain data - information about a specific AppDomain.
/// Size: 0x48 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpAppDomainData {
    pub AppDomainPtr: CLRDATA_ADDRESS,
    pub AppSecDesc: CLRDATA_ADDRESS,
    pub pLowFrequencyHeap: CLRDATA_ADDRESS,
    pub pHighFrequencyHeap: CLRDATA_ADDRESS,
    pub pStubHeap: CLRDATA_ADDRESS,
    pub DomainLocalBlock: CLRDATA_ADDRESS,
    pub pDomainLocalModules: CLRDATA_ADDRESS,
    pub dwId: u32,
    pub AssemblyCount: i32,
    pub FailedAssemblyCount: i32,
    pub appDomainStage: DacpAppDomainDataStage,
}

/// Assembly data - information about a loaded assembly.
/// Size: 0x40 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpAssemblyData {
    pub AssemblyPtr: CLRDATA_ADDRESS,
    pub ClassLoader: CLRDATA_ADDRESS,
    pub ParentDomain: CLRDATA_ADDRESS,
    pub DomainPtr: CLRDATA_ADDRESS,
    pub AssemblySecDesc: CLRDATA_ADDRESS,
    pub isDynamic: i32,
    pub ModuleCount: u32,
    pub LoadContext: u32,
    pub isDomainNeutral: i32,
    pub dwLocationFlags: u32,
}

/// Module data - information about a loaded module.
/// Size: 0xa0 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpModuleData {
    pub Address: CLRDATA_ADDRESS,
    pub PEAssembly: CLRDATA_ADDRESS,
    pub ilBase: CLRDATA_ADDRESS,
    pub metadataStart: CLRDATA_ADDRESS,
    pub metadataSize: u64,
    pub Assembly: CLRDATA_ADDRESS,
    pub bIsReflection: i32,
    pub bIsPEFile: i32,
    pub dwBaseClassIndex: u64,
    pub dwModuleID: u64,
    pub dwTransientFlags: u32,
    pub TypeDefToMethodTableMap: CLRDATA_ADDRESS,
    pub TypeRefToMethodTableMap: CLRDATA_ADDRESS,
    pub MethodDefToDescMap: CLRDATA_ADDRESS,
    pub FieldDefToDescMap: CLRDATA_ADDRESS,
    pub MemberRefToDescMap: CLRDATA_ADDRESS,
    pub FileReferencesMap: CLRDATA_ADDRESS,
    pub ManifestModuleReferencesMap: CLRDATA_ADDRESS,
    pub LoaderAllocator: CLRDATA_ADDRESS,
    pub ThunkHeap: CLRDATA_ADDRESS,
    pub dwModuleIndex: u64,
}

/// Thread data - information about a managed thread.
/// Size: 0x68 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpThreadData {
    pub corThreadId: u32,
    pub osThreadId: u32,
    pub state: i32,
    pub preemptiveGCDisabled: u32,
    pub allocContextPtr: CLRDATA_ADDRESS,
    pub allocContextLimit: CLRDATA_ADDRESS,
    pub context: CLRDATA_ADDRESS,
    pub domain: CLRDATA_ADDRESS,
    pub pFrame: CLRDATA_ADDRESS,
    pub lockCount: u32,
    pub firstNestedException: CLRDATA_ADDRESS,
    pub teb: CLRDATA_ADDRESS,
    pub fiberData: CLRDATA_ADDRESS,
    pub lastThrownObjectHandle: CLRDATA_ADDRESS,
    pub nextThread: CLRDATA_ADDRESS,
}

/// ReJIT data flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DacpReJitDataFlags {
    #[default]
    kUnknown = 0,
    kRequested = 1,
    kActive = 2,
    kReverted = 3,
}

/// ReJIT data - information about a rejitted method.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpReJitData {
    pub rejitID: CLRDATA_ADDRESS,
    pub flags: DacpReJitDataFlags,
    pub NativeCodeAddr: CLRDATA_ADDRESS,
}

/// MethodDesc data - information about a method.
/// Size: 0x98 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodDescData {
    pub bHasNativeCode: i32,
    pub bIsDynamic: i32,
    pub wSlotNumber: u16,
    pub NativeCodeAddr: CLRDATA_ADDRESS,
    pub AddressOfNativeCodeSlot: CLRDATA_ADDRESS,
    pub MethodDescPtr: CLRDATA_ADDRESS,
    pub MethodTablePtr: CLRDATA_ADDRESS,
    pub ModulePtr: CLRDATA_ADDRESS,
    pub MDToken: u32,
    pub GCInfo: CLRDATA_ADDRESS,
    pub GCStressCodeCopy: CLRDATA_ADDRESS,
    pub managedDynamicMethodObject: CLRDATA_ADDRESS,
    pub requestedIP: CLRDATA_ADDRESS,
    pub rejitDataCurrent: DacpReJitData,
    pub rejitDataRequested: DacpReJitData,
    pub cJittedRejitVersions: u32,
}

/// JIT types enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JITTypes {
    #[default]
    TYPE_UNKNOWN = 0,
    TYPE_JIT = 1,
    TYPE_PJIT = 2,
    TYPE_INTERPRETER = 3,
}

/// Code header data - information about JIT compiled code.
/// Size: 0x38 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpCodeHeaderData {
    pub GCInfo: CLRDATA_ADDRESS,
    pub JITType: JITTypes,
    pub MethodDescPtr: CLRDATA_ADDRESS,
    pub MethodStart: CLRDATA_ADDRESS,
    pub MethodSize: u32,
    pub ColdRegionStart: CLRDATA_ADDRESS,
    pub ColdRegionSize: u32,
    pub HotRegionSize: u32,
}

/// JIT manager info.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpJitManagerInfo {
    pub managerAddr: CLRDATA_ADDRESS,
    pub codeType: u32,
    pub ptrHeapList: CLRDATA_ADDRESS,
}

/// Threadpool data - information about the CLR thread pool.
/// Size: 0x58 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpThreadpoolData {
    pub cpuUtilization: i32,
    pub NumIdleWorkerThreads: i32,
    pub NumWorkingWorkerThreads: i32,
    pub NumRetiredWorkerThreads: i32,
    pub MinLimitTotalWorkerThreads: i32,
    pub MaxLimitTotalWorkerThreads: i32,
    pub FirstUnmanagedWorkRequest: CLRDATA_ADDRESS,
    pub HillClimbingLog: CLRDATA_ADDRESS,
    pub HillClimbingLogFirstIndex: i32,
    pub HillClimbingLogSize: i32,
    pub NumTimers: u32,
    pub NumCPThreads: i32,
    pub NumFreeCPThreads: i32,
    pub MaxFreeCPThreads: i32,
    pub NumRetiredCPThreads: i32,
    pub MaxLimitTotalCPThreads: i32,
    pub CurrentLimitTotalCPThreads: i32,
    pub MinLimitTotalCPThreads: i32,
    pub AsyncTimerCallbackCompletionFPtr: CLRDATA_ADDRESS,
}

/// Work request data.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpWorkRequestData {
    pub Function: CLRDATA_ADDRESS,
    pub Context: CLRDATA_ADDRESS,
    pub NextWorkRequest: CLRDATA_ADDRESS,
}

/// Hill climbing log entry.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpHillClimbingLogEntry {
    pub TickCount: u32,
    pub Transition: i32,
    pub NewControlSetting: i32,
    pub LastHistoryCount: i32,
    pub LastHistoryMean: f64,
}

/// Object type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DacpObjectType {
    #[default]
    OBJ_STRING = 0,
    OBJ_FREE = 1,
    OBJ_OBJECT = 2,
    OBJ_ARRAY = 3,
    OBJ_OTHER = 4,
}

/// Object data - information about a managed object.
/// Size: 0x60 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpObjectData {
    pub MethodTable: CLRDATA_ADDRESS,
    pub ObjectType: DacpObjectType,
    pub Size: u64,
    pub ElementTypeHandle: CLRDATA_ADDRESS,
    pub ElementType: u32,
    pub dwRank: u32,
    pub dwNumComponents: u64,
    pub dwComponentSize: u64,
    pub ArrayDataPtr: CLRDATA_ADDRESS,
    pub ArrayBoundsPtr: CLRDATA_ADDRESS,
    pub ArrayLowerBoundsPtr: CLRDATA_ADDRESS,
    pub RCW: CLRDATA_ADDRESS,
    pub CCW: CLRDATA_ADDRESS,
}

/// MethodTable data - information about a type's method table.
/// Size: 0x48 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodTableData {
    pub bIsFree: i32,
    pub Module: CLRDATA_ADDRESS,
    pub Class: CLRDATA_ADDRESS,
    pub ParentMethodTable: CLRDATA_ADDRESS,
    pub wNumInterfaces: u16,
    pub wNumMethods: u16,
    pub wNumVtableSlots: u16,
    pub wNumVirtuals: u16,
    pub BaseSize: u32,
    pub ComponentSize: u32,
    pub cl: u32,
    pub dwAttrClass: u32,
    pub bIsShared: i32,
    pub bIsDynamic: i32,
    pub bContainsPointers: i32,
}

/// MethodTable field data.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodTableFieldData {
    pub wNumInstanceFields: u16,
    pub wNumStaticFields: u16,
    pub wNumThreadStaticFields: u16,
    pub FirstField: CLRDATA_ADDRESS,
    pub wContextStaticOffset: u16,
    pub wContextStaticsSize: u16,
}

/// MethodTable transparency data.
/// Size: 0xc bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodTableTransparencyData {
    pub bHasCriticalTransparentInfo: i32,
    pub bIsCritical: i32,
    pub bIsTreatAsSafe: i32,
}

/// Field descriptor data.
/// Size: 0x40 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpFieldDescData {
    pub Type: u32,
    pub sigType: u32,
    pub MTOfType: CLRDATA_ADDRESS,
    pub ModuleOfType: CLRDATA_ADDRESS,
    pub TokenOfType: u32,
    pub mb: u32,
    pub MTOfEnclosingClass: CLRDATA_ADDRESS,
    pub dwOffset: u32,
    pub bIsThreadLocal: i32,
    pub bIsContextLocal: i32,
    pub bIsStatic: i32,
    pub NextField: CLRDATA_ADDRESS,
}

/// GC heap data - global GC information.
/// Size: 0x10 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGcHeapData {
    pub bServerMode: i32,
    pub bGcStructuresValid: i32,
    pub HeapCount: u32,
    pub g_max_generation: u32,
}

/// Generation data.
/// Size: 0x20 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGenerationData {
    pub start_segment: CLRDATA_ADDRESS,
    pub allocation_start: CLRDATA_ADDRESS,
    pub allocContextPtr: CLRDATA_ADDRESS,
    pub allocContextLimit: CLRDATA_ADDRESS,
}

/// GC heap details - detailed information about a GC heap.
/// Size: 0x120 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGcHeapDetails {
    pub heapAddr: CLRDATA_ADDRESS,
    pub alloc_allocated: CLRDATA_ADDRESS,
    pub mark_array: CLRDATA_ADDRESS,
    pub current_c_gc_state: CLRDATA_ADDRESS,
    pub next_sweep_obj: CLRDATA_ADDRESS,
    pub saved_sweep_ephemeral_seg: CLRDATA_ADDRESS,
    pub saved_sweep_ephemeral_start: CLRDATA_ADDRESS,
    pub background_saved_lowest_address: CLRDATA_ADDRESS,
    pub background_saved_highest_address: CLRDATA_ADDRESS,
    pub generation_table: [DacpGenerationData; DAC_NUMBERGENERATIONS],
    pub ephemeral_heap_segment: CLRDATA_ADDRESS,
    pub finalization_fill_pointers: [CLRDATA_ADDRESS; DAC_NUMBERGENERATIONS + 3],
    pub lowest_address: CLRDATA_ADDRESS,
    pub highest_address: CLRDATA_ADDRESS,
    pub card_table: CLRDATA_ADDRESS,
}

/// Heap segment data.
/// Size: 0x58 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpHeapSegmentData {
    pub segmentAddr: CLRDATA_ADDRESS,
    pub allocated: CLRDATA_ADDRESS,
    pub committed: CLRDATA_ADDRESS,
    pub reserved: CLRDATA_ADDRESS,
    pub used: CLRDATA_ADDRESS,
    pub mem: CLRDATA_ADDRESS,
    pub next: CLRDATA_ADDRESS,
    pub gc_heap: CLRDATA_ADDRESS,
    pub highAllocMark: CLRDATA_ADDRESS,
    pub flags: usize,
    pub background_allocated: CLRDATA_ADDRESS,
}

/// OOM (Out of Memory) data.
/// Size: 0x38 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpOomData {
    pub reason: i32,
    pub alloc_size: u64,
    pub available_pagefile_mb: u64,
    pub gc_index: u64,
    pub fgm: i32,
    pub size: u64,
    pub loh_p: i32,
}

/// GC heap analyze data.
/// Size: 0x20 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGcHeapAnalyzeData {
    pub heapAddr: CLRDATA_ADDRESS,
    pub internal_root_array: CLRDATA_ADDRESS,
    pub internal_root_array_index: u64,
    pub heap_analyze_success: i32,
}

/// Domain local module data.
/// Size: 0x30 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpDomainLocalModuleData {
    pub appDomainAddr: CLRDATA_ADDRESS,
    pub ModuleID: u64,
    pub pClassData: CLRDATA_ADDRESS,
    pub pDynamicClassTable: CLRDATA_ADDRESS,
    pub pGCStaticDataStart: CLRDATA_ADDRESS,
    pub pNonGCStaticDataStart: CLRDATA_ADDRESS,
}

/// Thread local module data.
/// Size: 0x30 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpThreadLocalModuleData {
    pub threadAddr: CLRDATA_ADDRESS,
    pub ModuleIndex: u64,
    pub pClassData: CLRDATA_ADDRESS,
    pub pDynamicClassTable: CLRDATA_ADDRESS,
    pub pGCStaticDataStart: CLRDATA_ADDRESS,
    pub pNonGCStaticDataStart: CLRDATA_ADDRESS,
}

/// SyncBlock data.
/// Size: 0x48 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpSyncBlockData {
    pub Object: CLRDATA_ADDRESS,
    pub bFree: i32,
    pub SyncBlockPointer: CLRDATA_ADDRESS,
    pub COMFlags: u32,
    pub MonitorHeld: u32,
    pub Recursion: u32,
    pub HoldingThread: CLRDATA_ADDRESS,
    pub AdditionalThreadCount: u32,
    pub appDomainPtr: CLRDATA_ADDRESS,
    pub SyncBlockCount: u32,
}

/// SyncBlock cleanup data.
/// Size: 0x28 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpSyncBlockCleanupData {
    pub SyncBlockPointer: CLRDATA_ADDRESS,
    pub nextSyncBlock: CLRDATA_ADDRESS,
    pub blockRCW: CLRDATA_ADDRESS,
    pub blockClassFactory: CLRDATA_ADDRESS,
    pub blockCCW: CLRDATA_ADDRESS,
}

/// RCW (Runtime Callable Wrapper) data.
/// Size: 0x58 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpRCWData {
    pub identityPointer: CLRDATA_ADDRESS,
    pub unknownPointer: CLRDATA_ADDRESS,
    pub managedObject: CLRDATA_ADDRESS,
    pub jupiterObject: CLRDATA_ADDRESS,
    pub vtablePtr: CLRDATA_ADDRESS,
    pub creatorThread: CLRDATA_ADDRESS,
    pub ctxCookie: CLRDATA_ADDRESS,
    pub refCount: i32,
    pub interfaceCount: i32,
    pub isJupiterObject: i32,
    pub supportsIInspectable: i32,
    pub isAggregated: i32,
    pub isContained: i32,
    pub isFreeThreaded: i32,
    pub isDisconnected: i32,
}

/// CCW (COM Callable Wrapper) data.
/// Size: 0x48 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpCCWData {
    pub outerIUnknown: CLRDATA_ADDRESS,
    pub managedObject: CLRDATA_ADDRESS,
    pub handle: CLRDATA_ADDRESS,
    pub ccwAddress: CLRDATA_ADDRESS,
    pub refCount: i32,
    pub interfaceCount: i32,
    pub isNeutered: i32,
    pub jupiterRefCount: i32,
    pub isPegged: i32,
    pub isGlobalPegged: i32,
    pub hasStrongRef: i32,
    pub isExtendsCOMObject: i32,
    pub isAggregated: i32,
}

/// COM interface pointer data.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpCOMInterfacePointerData {
    pub methodTable: CLRDATA_ADDRESS,
    pub interfacePtr: CLRDATA_ADDRESS,
    pub comContext: CLRDATA_ADDRESS,
}

/// Useful globals data.
/// Size: 0x28 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpUsefulGlobalsData {
    pub ArrayMethodTable: CLRDATA_ADDRESS,
    pub StringMethodTable: CLRDATA_ADDRESS,
    pub ObjectMethodTable: CLRDATA_ADDRESS,
    pub ExceptionMethodTable: CLRDATA_ADDRESS,
    pub FreeMethodTable: CLRDATA_ADDRESS,
}

/// Allocation data.
/// Size: 0x10 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpAllocData {
    pub allocBytes: CLRDATA_ADDRESS,
    pub allocBytesLoh: CLRDATA_ADDRESS,
}

/// Generation allocation data.
/// Size: 0x40 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGenerationAllocData {
    pub allocData: [DacpAllocData; DAC_NUMBERGENERATIONS],
}

/// Code heap type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeHeapType {
    #[default]
    CODEHEAP_LOADER = 0,
    CODEHEAP_HOST = 1,
    CODEHEAP_UNKNOWN = 2,
}

/// JIT code heap info.
/// Size: 0x18 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpJitCodeHeapInfo {
    pub codeHeapType: u32,
    pub address1: CLRDATA_ADDRESS,
    pub address2: CLRDATA_ADDRESS,
}

/// MethodDesc transparency data.
/// Size: 0xc bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodDescTransparencyData {
    pub bHasCriticalTransparentInfo: i32,
    pub bIsCritical: i32,
    pub bIsTreatAsSafe: i32,
}

/// Exception object data.
/// Size: 0x38 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpExceptionObjectData {
    pub Message: CLRDATA_ADDRESS,
    pub InnerException: CLRDATA_ADDRESS,
    pub StackTrace: CLRDATA_ADDRESS,
    pub WatsonBuckets: CLRDATA_ADDRESS,
    pub StackTraceString: CLRDATA_ADDRESS,
    pub RemoteStackTraceString: CLRDATA_ADDRESS,
    pub HResult: i32,
    pub XCode: i32,
}

/// GC interesting info data constants.
pub const DAC_NUM_GC_DATA_POINTS: usize = 9;
pub const DAC_MAX_COMPACT_REASONS_COUNT: usize = 11;
pub const DAC_MAX_EXPAND_MECHANISMS_COUNT: usize = 6;
pub const DAC_MAX_GC_MECHANISM_BITS_COUNT: usize = 2;
pub const DAC_MAX_GLOBAL_GC_MECHANISMS_COUNT: usize = 6;

/// GC interesting info data.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpGCInterestingInfoData {
    pub interestingDataPoints: [usize; DAC_NUM_GC_DATA_POINTS],
    pub compactReasons: [usize; DAC_MAX_COMPACT_REASONS_COUNT],
    pub expandMechanisms: [usize; DAC_MAX_EXPAND_MECHANISMS_COUNT],
    pub bitMechanisms: [usize; DAC_MAX_GC_MECHANISM_BITS_COUNT],
    pub globalMechanisms: [usize; DAC_MAX_GLOBAL_GC_MECHANISMS_COUNT],
}

/// Optimization tier enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizationTier {
    #[default]
    OptimizationTier_Unknown = 0,
    OptimizationTier_MinOptJitted = 1,
    OptimizationTier_Optimized = 2,
    OptimizationTier_QuickJitted = 3,
    OptimizationTier_OptimizedTier1 = 4,
    OptimizationTier_ReadyToRun = 5,
    OptimizationTier_OptimizedTier1OSR = 6,
    OptimizationTier_QuickJittedInstrumented = 7,
    OptimizationTier_OptimizedTier1Instrumented = 8,
}

/// Tiered version data.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpTieredVersionData {
    pub NativeCodeAddr: CLRDATA_ADDRESS,
    pub OptimizationTier: OptimizationTier,
    pub NativeCodeVersionNodePtr: CLRDATA_ADDRESS,
}

/// MethodTable collectible data.
/// Size: 0x10 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpMethodTableCollectibleData {
    pub LoaderAllocatorObjectHandle: CLRDATA_ADDRESS,
    pub bCollectible: i32,
}

/// ReJIT data 2 flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DacpReJitData2Flags {
    #[default]
    kUnknown = 0,
    kRequested = 1,
    kActive = 2,
    kReverted = 3,
}

/// ReJIT data 2.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpReJitData2 {
    pub rejitID: u32,
    pub flags: DacpReJitData2Flags,
    pub il: CLRDATA_ADDRESS,
    pub ilCodeVersionNodePtr: CLRDATA_ADDRESS,
}

/// Profiler IL modification type.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerILModificationType {
    #[default]
    Unmodified = 0,
    ILModified = 1,
    ReJITModified = 2,
}

/// Profiler IL data.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DacpProfilerILData {
    pub modificationType: ProfilerILModificationType,
    pub il: CLRDATA_ADDRESS,
    pub rejitID: u32,
}

/// EH clause type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EHClauseType {
    #[default]
    EHFault = 0,
    EHFinally = 1,
    EHFilter = 2,
    EHTyped = 3,
    EHUnknown = 4,
}

/// Exception handling info.
/// Size: 0x58 bytes
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DACEHInfo {
    pub clauseType: EHClauseType,
    pub tryStartOffset: CLRDATA_ADDRESS,
    pub tryEndOffset: CLRDATA_ADDRESS,
    pub handlerStartOffset: CLRDATA_ADDRESS,
    pub handlerEndOffset: CLRDATA_ADDRESS,
    pub isDuplicateClause: i32,
    pub filterOffset: CLRDATA_ADDRESS,
    pub isCatchAllHandler: i32,
    pub moduleAddr: CLRDATA_ADDRESS,
    pub mtCatch: CLRDATA_ADDRESS,
    pub tokCatch: u32,
}

/// Module map type enumeration.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleMapType {
    TYPEDEFTOMETHODTABLE = 0,
    TYPEREFTOMETHODTABLE = 1,
}

/// Virtual call stub heap type.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VCSHeapType {
    IndcellHeap = 0,
    LookupHeap = 1,
    ResolveHeap = 2,
    DispatchHeap = 3,
    CacheEntryHeap = 4,
    VtableHeap = 5,
}

/// Loader heap kind.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaderHeapKind {
    LoaderHeapKindNormal = 0,
    LoaderHeapKindExplicitControl = 1,
}

/// Method table initialization flags.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodTableInitializationFlags {
    MethodTableInitialized = 1,
    MethodTableInitializationFailed = 2,
}

/// Callback for module map traversal.
pub type MODULEMAPTRAVERSE = Option<
    unsafe extern "system" fn(
        index: u32,
        methodTable: CLRDATA_ADDRESS,
        token: *mut std::ffi::c_void,
    ),
>;

/// Callback for heap visiting.
pub type VISITHEAP = Option<
    unsafe extern "system" fn(
        blockData: CLRDATA_ADDRESS,
        blockSize: usize,
        blockIsCurrentBlock: i32,
    ),
>;

/// Callback for RCW cleanup traversal.
pub type VISITRCWFORCLEANUP = Option<
    unsafe extern "system" fn(
        RCW: CLRDATA_ADDRESS,
        Context: CLRDATA_ADDRESS,
        Thread: CLRDATA_ADDRESS,
        bIsFreeThreaded: i32,
        token: *mut std::ffi::c_void,
    ) -> i32,
>;

/// Callback for exception handler info dumping.
pub type DUMPEHINFO = Option<
    unsafe extern "system" fn(
        clauseIndex: u32,
        totalClauses: u32,
        pEHInfo: *mut DACEHInfo,
        token: *mut std::ffi::c_void,
    ) -> i32,
>;

/// ISOSDacInterface - Main SOS debugging interface.
/// Provides methods for querying CLR runtime internal data.
#[interface("436f00f2-b42a-4b9f-870c-e73db66ae930")]
pub unsafe trait ISOSDacInterface: IUnknown {
    // ThreadStore
    pub unsafe fn GetThreadStoreData(&self, data: *mut DacpThreadStoreData) -> HRESULT;

    // AppDomains
    pub unsafe fn GetAppDomainStoreData(&self, data: *mut DacpAppDomainStoreData) -> HRESULT;
    pub unsafe fn GetAppDomainList(
        &self,
        count: u32,
        values: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetAppDomainData(
        &self,
        addr: CLRDATA_ADDRESS,
        data: *mut DacpAppDomainData,
    ) -> HRESULT;
    pub unsafe fn GetAppDomainName(
        &self,
        addr: CLRDATA_ADDRESS,
        count: u32,
        name: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetDomainFromContext(
        &self,
        context: CLRDATA_ADDRESS,
        domain: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // Assemblies
    pub unsafe fn GetAssemblyList(
        &self,
        appDomain: CLRDATA_ADDRESS,
        count: i32,
        values: *mut CLRDATA_ADDRESS,
        pNeeded: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyData(
        &self,
        baseDomainPtr: CLRDATA_ADDRESS,
        assembly: CLRDATA_ADDRESS,
        data: *mut DacpAssemblyData,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyName(
        &self,
        assembly: CLRDATA_ADDRESS,
        count: u32,
        name: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;

    // Modules
    pub unsafe fn GetModule(
        &self,
        addr: CLRDATA_ADDRESS,
        module: *mut *mut IXCLRDataModule_SOS,
    ) -> HRESULT;
    pub unsafe fn GetModuleData(
        &self,
        moduleAddr: CLRDATA_ADDRESS,
        data: *mut DacpModuleData,
    ) -> HRESULT;
    pub unsafe fn TraverseModuleMap(
        &self,
        mmt: ModuleMapType,
        moduleAddr: CLRDATA_ADDRESS,
        pCallback: MODULEMAPTRAVERSE,
        token: *mut std::ffi::c_void,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyModuleList(
        &self,
        assembly: CLRDATA_ADDRESS,
        count: u32,
        modules: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetILForModule(
        &self,
        moduleAddr: CLRDATA_ADDRESS,
        rva: u32,
        il: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // Threads
    pub unsafe fn GetThreadData(
        &self,
        thread: CLRDATA_ADDRESS,
        data: *mut DacpThreadData,
    ) -> HRESULT;
    pub unsafe fn GetThreadFromThinlockID(
        &self,
        thinLockId: u32,
        pThread: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetStackLimits(
        &self,
        threadPtr: CLRDATA_ADDRESS,
        lower: *mut CLRDATA_ADDRESS,
        upper: *mut CLRDATA_ADDRESS,
        fp: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // MethodDescs
    pub unsafe fn GetMethodDescData(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        ip: CLRDATA_ADDRESS,
        data: *mut DacpMethodDescData,
        cRevertedRejitVersions: u32,
        rgRevertedRejitData: *mut DacpReJitData,
        pcNeededRevertedRejitData: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMethodDescPtrFromIP(
        &self,
        ip: CLRDATA_ADDRESS,
        ppMD: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetMethodDescName(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        count: u32,
        name: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMethodDescPtrFromFrame(
        &self,
        frameAddr: CLRDATA_ADDRESS,
        ppMD: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetMethodDescFromToken(
        &self,
        moduleAddr: CLRDATA_ADDRESS,
        token: u32,
        methodDesc: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetMethodDescTransparencyData(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        data: *mut DacpMethodDescTransparencyData,
    ) -> HRESULT;

    // JIT Data
    pub unsafe fn GetCodeHeaderData(
        &self,
        ip: CLRDATA_ADDRESS,
        data: *mut DacpCodeHeaderData,
    ) -> HRESULT;
    pub unsafe fn GetJitManagerList(
        &self,
        count: u32,
        managers: *mut DacpJitManagerInfo,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetJitHelperFunctionName(
        &self,
        ip: CLRDATA_ADDRESS,
        count: u32,
        name: *mut u8,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetJumpThunkTarget(
        &self,
        ctx: *mut std::ffi::c_void,
        targetIP: *mut CLRDATA_ADDRESS,
        targetMD: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // ThreadPool
    pub unsafe fn GetThreadpoolData(&self, data: *mut DacpThreadpoolData) -> HRESULT;
    pub unsafe fn GetWorkRequestData(
        &self,
        addrWorkRequest: CLRDATA_ADDRESS,
        data: *mut DacpWorkRequestData,
    ) -> HRESULT;
    pub unsafe fn GetHillClimbingLogEntry(
        &self,
        addr: CLRDATA_ADDRESS,
        data: *mut DacpHillClimbingLogEntry,
    ) -> HRESULT;

    // Objects
    pub unsafe fn GetObjectData(
        &self,
        objAddr: CLRDATA_ADDRESS,
        data: *mut DacpObjectData,
    ) -> HRESULT;
    pub unsafe fn GetObjectStringData(
        &self,
        obj: CLRDATA_ADDRESS,
        count: u32,
        stringData: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetObjectClassName(
        &self,
        obj: CLRDATA_ADDRESS,
        count: u32,
        className: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;

    // MethodTable
    pub unsafe fn GetMethodTableName(
        &self,
        mt: CLRDATA_ADDRESS,
        count: u32,
        mtName: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMethodTableData(
        &self,
        mt: CLRDATA_ADDRESS,
        data: *mut DacpMethodTableData,
    ) -> HRESULT;
    pub unsafe fn GetMethodTableSlot(
        &self,
        mt: CLRDATA_ADDRESS,
        slot: u32,
        value: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetMethodTableFieldData(
        &self,
        mt: CLRDATA_ADDRESS,
        data: *mut DacpMethodTableFieldData,
    ) -> HRESULT;
    pub unsafe fn GetMethodTableTransparencyData(
        &self,
        mt: CLRDATA_ADDRESS,
        data: *mut DacpMethodTableTransparencyData,
    ) -> HRESULT;

    // EEClass
    pub unsafe fn GetMethodTableForEEClass(
        &self,
        eeClass: CLRDATA_ADDRESS,
        value: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // FieldDesc
    pub unsafe fn GetFieldDescData(
        &self,
        fieldDesc: CLRDATA_ADDRESS,
        data: *mut DacpFieldDescData,
    ) -> HRESULT;

    // Frames
    pub unsafe fn GetFrameName(
        &self,
        vtable: CLRDATA_ADDRESS,
        count: u32,
        frameName: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;

    // PEFiles
    pub unsafe fn GetPEFileBase(
        &self,
        addr: CLRDATA_ADDRESS,
        base: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetPEFileName(
        &self,
        addr: CLRDATA_ADDRESS,
        count: u32,
        fileName: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;

    // GC
    pub unsafe fn GetGCHeapData(&self, data: *mut DacpGcHeapData) -> HRESULT;
    pub unsafe fn GetGCHeapList(
        &self,
        count: u32,
        heaps: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetGCHeapDetails(
        &self,
        heap: CLRDATA_ADDRESS,
        details: *mut DacpGcHeapDetails,
    ) -> HRESULT;
    pub unsafe fn GetGCHeapStaticData(&self, data: *mut DacpGcHeapDetails) -> HRESULT;
    pub unsafe fn GetHeapSegmentData(
        &self,
        seg: CLRDATA_ADDRESS,
        data: *mut DacpHeapSegmentData,
    ) -> HRESULT;
    pub unsafe fn GetOOMData(&self, oomAddr: CLRDATA_ADDRESS, data: *mut DacpOomData) -> HRESULT;
    pub unsafe fn GetOOMStaticData(&self, data: *mut DacpOomData) -> HRESULT;
    pub unsafe fn GetHeapAnalyzeData(
        &self,
        addr: CLRDATA_ADDRESS,
        data: *mut DacpGcHeapAnalyzeData,
    ) -> HRESULT;
    pub unsafe fn GetHeapAnalyzeStaticData(&self, data: *mut DacpGcHeapAnalyzeData) -> HRESULT;

    // DomainLocal
    pub unsafe fn GetDomainLocalModuleData(
        &self,
        addr: CLRDATA_ADDRESS,
        data: *mut DacpDomainLocalModuleData,
    ) -> HRESULT;
    pub unsafe fn GetDomainLocalModuleDataFromAppDomain(
        &self,
        appDomainAddr: CLRDATA_ADDRESS,
        moduleID: i32,
        data: *mut DacpDomainLocalModuleData,
    ) -> HRESULT;
    pub unsafe fn GetDomainLocalModuleDataFromModule(
        &self,
        moduleAddr: CLRDATA_ADDRESS,
        data: *mut DacpDomainLocalModuleData,
    ) -> HRESULT;

    // ThreadLocal
    pub unsafe fn GetThreadLocalModuleData(
        &self,
        thread: CLRDATA_ADDRESS,
        index: u32,
        data: *mut DacpThreadLocalModuleData,
    ) -> HRESULT;

    // SyncBlock
    pub unsafe fn GetSyncBlockData(&self, number: u32, data: *mut DacpSyncBlockData) -> HRESULT;
    pub unsafe fn GetSyncBlockCleanupData(
        &self,
        addr: CLRDATA_ADDRESS,
        data: *mut DacpSyncBlockCleanupData,
    ) -> HRESULT;

    // Handles
    pub unsafe fn GetHandleEnum(&self, ppHandleEnum: *mut *mut ISOSHandleEnum) -> HRESULT;
    pub unsafe fn GetHandleEnumForTypes(
        &self,
        types: *const u32,
        count: u32,
        ppHandleEnum: *mut *mut ISOSHandleEnum,
    ) -> HRESULT;
    pub unsafe fn GetHandleEnumForGC(
        &self,
        generation: u32,
        ppHandleEnum: *mut *mut ISOSHandleEnum,
    ) -> HRESULT;

    // EH
    pub unsafe fn TraverseEHInfo(
        &self,
        ip: CLRDATA_ADDRESS,
        pCallback: DUMPEHINFO,
        token: *mut std::ffi::c_void,
    ) -> HRESULT;
    pub unsafe fn GetNestedExceptionData(
        &self,
        exception: CLRDATA_ADDRESS,
        exceptionObject: *mut CLRDATA_ADDRESS,
        nextNestedException: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    // StressLog
    pub unsafe fn GetStressLogAddress(&self, stressLog: *mut CLRDATA_ADDRESS) -> HRESULT;

    // Heaps
    pub unsafe fn TraverseLoaderHeap(
        &self,
        loaderHeapAddr: CLRDATA_ADDRESS,
        pCallback: VISITHEAP,
    ) -> HRESULT;
    pub unsafe fn GetCodeHeapList(
        &self,
        jitManager: CLRDATA_ADDRESS,
        count: u32,
        codeHeaps: *mut DacpJitCodeHeapInfo,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn TraverseVirtCallStubHeap(
        &self,
        pAppDomain: CLRDATA_ADDRESS,
        heaptype: VCSHeapType,
        pCallback: VISITHEAP,
    ) -> HRESULT;

    // Other
    pub unsafe fn GetUsefulGlobals(&self, data: *mut DacpUsefulGlobalsData) -> HRESULT;
    pub unsafe fn GetClrWatsonBuckets(
        &self,
        thread: CLRDATA_ADDRESS,
        pGenericModeBlock: *mut std::ffi::c_void,
    ) -> HRESULT;
    pub unsafe fn GetTLSIndex(&self, pIndex: *mut u32) -> HRESULT;
    pub unsafe fn GetDacModuleHandle(&self, phModule: *mut *mut std::ffi::c_void) -> HRESULT;

    // COM
    pub unsafe fn GetRCWData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpRCWData) -> HRESULT;
    pub unsafe fn GetRCWInterfaces(
        &self,
        rcw: CLRDATA_ADDRESS,
        count: u32,
        interfaces: *mut DacpCOMInterfacePointerData,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetCCWData(&self, ccw: CLRDATA_ADDRESS, data: *mut DacpCCWData) -> HRESULT;
    pub unsafe fn GetCCWInterfaces(
        &self,
        ccw: CLRDATA_ADDRESS,
        count: u32,
        interfaces: *mut DacpCOMInterfacePointerData,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn TraverseRCWCleanupList(
        &self,
        cleanupListPtr: CLRDATA_ADDRESS,
        pCallback: VISITRCWFORCLEANUP,
        token: *mut std::ffi::c_void,
    ) -> HRESULT;

    // GC Reference Functions
    pub unsafe fn GetStackReferences(
        &self,
        osThreadID: u32,
        ppEnum: *mut *mut ISOSStackRefEnum,
    ) -> HRESULT;
    pub unsafe fn GetRegisterName(
        &self,
        regName: i32,
        count: u32,
        buffer: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;

    pub unsafe fn GetThreadAllocData(
        &self,
        thread: CLRDATA_ADDRESS,
        data: *mut DacpAllocData,
    ) -> HRESULT;
    pub unsafe fn GetHeapAllocData(
        &self,
        count: u32,
        data: *mut DacpGenerationAllocData,
        pNeeded: *mut u32,
    ) -> HRESULT;

    // BindingDisplay
    pub unsafe fn GetFailedAssemblyList(
        &self,
        appDomain: CLRDATA_ADDRESS,
        count: i32,
        values: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetPrivateBinPaths(
        &self,
        appDomain: CLRDATA_ADDRESS,
        count: i32,
        paths: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyLocation(
        &self,
        assembly: CLRDATA_ADDRESS,
        count: i32,
        location: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetAppDomainConfigFile(
        &self,
        appDomain: CLRDATA_ADDRESS,
        count: i32,
        configFile: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetApplicationBase(
        &self,
        appDomain: CLRDATA_ADDRESS,
        count: i32,
        base: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFailedAssemblyData(
        &self,
        assembly: CLRDATA_ADDRESS,
        pContext: *mut u32,
        pResult: *mut HRESULT,
    ) -> HRESULT;
    pub unsafe fn GetFailedAssemblyLocation(
        &self,
        assembly: CLRDATA_ADDRESS,
        count: u32,
        location: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFailedAssemblyDisplayName(
        &self,
        assembly: CLRDATA_ADDRESS,
        count: u32,
        name: *mut u16,
        pNeeded: *mut u32,
    ) -> HRESULT;
}

/// ISOSDacInterface2 - Extended SOS debugging interface.
#[interface("A16026EC-96F4-40BA-87FB-5575986FB7AF")]
pub unsafe trait ISOSDacInterface2: IUnknown {
    pub unsafe fn GetObjectExceptionData(
        &self,
        objAddr: CLRDATA_ADDRESS,
        data: *mut DacpExceptionObjectData,
    ) -> HRESULT;
    pub unsafe fn IsRCWDCOMProxy(&self, rcwAddr: CLRDATA_ADDRESS, isDCOMProxy: *mut i32)
    -> HRESULT;
}

/// ISOSDacInterface3 - GC interesting info interface.
#[interface("B08C5CDC-FD8A-49C5-AB38-5FEEF35235B4")]
pub unsafe trait ISOSDacInterface3: IUnknown {
    pub unsafe fn GetGCInterestingInfoData(
        &self,
        interestingInfoAddr: CLRDATA_ADDRESS,
        data: *mut DacpGCInterestingInfoData,
    ) -> HRESULT;
    pub unsafe fn GetGCInterestingInfoStaticData(
        &self,
        data: *mut DacpGCInterestingInfoData,
    ) -> HRESULT;
    pub unsafe fn GetGCGlobalMechanisms(&self, globalMechanisms: *mut usize) -> HRESULT;
}

/// ISOSDacInterface4 - CLR notification interface.
#[interface("74B9D34C-A612-4B07-93DD-5462178FCE11")]
pub unsafe trait ISOSDacInterface4: IUnknown {
    pub unsafe fn GetClrNotification(
        &self,
        arguments: *mut CLRDATA_ADDRESS,
        count: i32,
        pNeeded: *mut i32,
    ) -> HRESULT;
}

/// ISOSDacInterface5 - Tiered compilation interface.
#[interface("127d6abe-6c86-4e48-8e7b-220781c58101")]
pub unsafe trait ISOSDacInterface5: IUnknown {
    pub unsafe fn GetTieredVersions(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        rejitId: i32,
        nativeCodeAddrs: *mut DacpTieredVersionData,
        cNativeCodeAddrs: i32,
        pcNativeCodeAddrs: *mut i32,
    ) -> HRESULT;
}

/// ISOSDacInterface6 - Collectible method table interface.
#[interface("11206399-4B66-4EDB-98EA-85654E59AD45")]
pub unsafe trait ISOSDacInterface6: IUnknown {
    pub unsafe fn GetMethodTableCollectibleData(
        &self,
        mt: CLRDATA_ADDRESS,
        data: *mut DacpMethodTableCollectibleData,
    ) -> HRESULT;
}

/// ISOSDacInterface7 - ReJIT and profiler interface.
#[interface("c1020dde-fe98-4536-a53b-f35a74c327eb")]
pub unsafe trait ISOSDacInterface7: IUnknown {
    pub unsafe fn GetPendingReJITID(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        pRejitId: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetReJITInformation(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        rejitId: i32,
        pRejitData: *mut DacpReJitData2,
    ) -> HRESULT;
    pub unsafe fn GetProfilerModifiedILInformation(
        &self,
        methodDesc: CLRDATA_ADDRESS,
        pILData: *mut DacpProfilerILData,
    ) -> HRESULT;
    pub unsafe fn GetMethodsWithProfilerModifiedIL(
        &self,
        module: CLRDATA_ADDRESS,
        methodDescs: *mut CLRDATA_ADDRESS,
        cMethodDescs: i32,
        pcMethodDescs: *mut i32,
    ) -> HRESULT;
}

/// ISOSDacInterface8 - Generation and finalization interface.
#[interface("c12f35a9-e55c-4520-a894-b3dc5165dfce")]
pub unsafe trait ISOSDacInterface8: IUnknown {
    pub unsafe fn GetNumberGenerations(&self, pGenerations: *mut u32) -> HRESULT;
    pub unsafe fn GetGenerationTable(
        &self,
        cGenerations: u32,
        pGenerationData: *mut DacpGenerationData,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFinalizationFillPointers(
        &self,
        cFillPointers: u32,
        pFinalizationFillPointers: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetGenerationTableSvr(
        &self,
        heapAddr: CLRDATA_ADDRESS,
        cGenerations: u32,
        pGenerationData: *mut DacpGenerationData,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFinalizationFillPointersSvr(
        &self,
        heapAddr: CLRDATA_ADDRESS,
        cFillPointers: u32,
        pFinalizationFillPointers: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyLoadContext(
        &self,
        methodTable: CLRDATA_ADDRESS,
        assemblyLoadContext: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
}

/// SOS breaking change version constant.
pub const SOS_BREAKING_CHANGE_VERSION: i32 = 5;

/// ISOSDacInterface9 - Breaking change version interface.
#[interface("4eca42d8-7e7b-4c8a-a116-7bfbf6929267")]
pub unsafe trait ISOSDacInterface9: IUnknown {
    pub unsafe fn GetBreakingChangeVersion(&self, pVersion: *mut i32) -> HRESULT;
}

/// ISOSDacInterface10 - ComWrappers interface.
#[interface("90B8FCC3-7251-4B0A-AE3D-5C13A67EC9AA")]
pub unsafe trait ISOSDacInterface10: IUnknown {
    pub unsafe fn GetObjectComWrappersData(
        &self,
        objAddr: CLRDATA_ADDRESS,
        rcw: *mut CLRDATA_ADDRESS,
        count: u32,
        mowList: *mut CLRDATA_ADDRESS,
        pNeeded: *mut u32,
    ) -> HRESULT;
    pub unsafe fn IsComWrappersCCW(
        &self,
        ccw: CLRDATA_ADDRESS,
        isComWrappersCCW: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetComWrappersCCWData(
        &self,
        ccw: CLRDATA_ADDRESS,
        managedObject: *mut CLRDATA_ADDRESS,
        refCount: *mut i32,
    ) -> HRESULT;
    pub unsafe fn IsComWrappersRCW(
        &self,
        rcw: CLRDATA_ADDRESS,
        isComWrappersRCW: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetComWrappersRCWData(
        &self,
        rcw: CLRDATA_ADDRESS,
        identity: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
}

/// ISOSDacInterface11 - Tagged memory interface.
#[interface("96BA1DB9-14CD-4492-8065-1CAAECF6E5CF")]
pub unsafe trait ISOSDacInterface11: IUnknown {
    pub unsafe fn IsTrackedType(
        &self,
        objAddr: CLRDATA_ADDRESS,
        isTrackedType: *mut i32,
        hasTaggedMemory: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetTaggedMemory(
        &self,
        objAddr: CLRDATA_ADDRESS,
        taggedMemory: *mut CLRDATA_ADDRESS,
        taggedMemorySizeInBytes: *mut usize,
    ) -> HRESULT;
}

/// ISOSDacInterface12 - Global allocation context interface.
#[interface("1b93bacc-8ca4-432d-943a-3e6e7ec0b0a3")]
pub unsafe trait ISOSDacInterface12: IUnknown {
    pub unsafe fn GetGlobalAllocationContext(
        &self,
        allocPtr: *mut CLRDATA_ADDRESS,
        allocLimit: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
}

/// ISOSDacInterface13 - Extended loader heap and memory region interface.
#[interface("3176a8ed-597b-4f54-a71f-83695c6a8c5e")]
pub unsafe trait ISOSDacInterface13: IUnknown {
    pub unsafe fn TraverseLoaderHeap(
        &self,
        loaderHeapAddr: CLRDATA_ADDRESS,
        kind: LoaderHeapKind,
        pCallback: VISITHEAP,
    ) -> HRESULT;
    pub unsafe fn GetDomainLoaderAllocator(
        &self,
        domainAddress: CLRDATA_ADDRESS,
        pLoaderAllocator: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetLoaderAllocatorHeapNames(
        &self,
        count: i32,
        ppNames: *mut *const i8,
        pNeeded: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetLoaderAllocatorHeaps(
        &self,
        loaderAllocator: CLRDATA_ADDRESS,
        count: i32,
        pLoaderHeaps: *mut CLRDATA_ADDRESS,
        pKinds: *mut LoaderHeapKind,
        pNeeded: *mut i32,
    ) -> HRESULT;
    pub unsafe fn GetHandleTableMemoryRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum) -> HRESULT;
    pub unsafe fn GetGCBookkeepingMemoryRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum)
    -> HRESULT;
    pub unsafe fn GetGCFreeRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum) -> HRESULT;
    pub unsafe fn LockedFlush(&self) -> HRESULT;
}

/// ISOSDacInterface14 - Static base address interface.
#[interface("9aa22aca-6dc6-4a0c-b4e0-70d2416b9837")]
pub unsafe trait ISOSDacInterface14: IUnknown {
    pub unsafe fn GetStaticBaseAddress(
        &self,
        methodTable: CLRDATA_ADDRESS,
        nonGCStaticsAddress: *mut CLRDATA_ADDRESS,
        GCStaticsAddress: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetThreadStaticBaseAddress(
        &self,
        methodTable: CLRDATA_ADDRESS,
        thread: CLRDATA_ADDRESS,
        nonGCStaticsAddress: *mut CLRDATA_ADDRESS,
        GCStaticsAddress: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;
    pub unsafe fn GetMethodTableInitializationFlags(
        &self,
        methodTable: CLRDATA_ADDRESS,
        initializationStatus: *mut MethodTableInitializationFlags,
    ) -> HRESULT;
}

/// ISOSDacInterface15 - Method table slot enumerator interface.
#[interface("7ed81261-52a9-4a23-a358-c3313dea30a8")]
pub unsafe trait ISOSDacInterface15: IUnknown {
    pub unsafe fn GetMethodTableSlotEnumerator(
        &self,
        mt: CLRDATA_ADDRESS,
        enumerator: *mut *mut ISOSMethodEnum,
    ) -> HRESULT;
}

/// ISOSDacInterface16 - GC dynamic adaptation mode interface.
#[interface("4ba12ff8-daac-4e43-ac56-98cf8d5c595d")]
pub unsafe trait ISOSDacInterface16: IUnknown {
    pub unsafe fn GetGCDynamicAdaptationMode(&self, pDynamicAdaptationMode: *mut i32) -> HRESULT;
}
