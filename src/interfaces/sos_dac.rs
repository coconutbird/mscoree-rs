//! ISOSDacInterface definitions for SOS debugging.
//!
//! These interfaces provide methods for SOS (Son of Strike) debugger extension
//! to query the CLR runtime for internal data.
//!
//! Available since .NET Framework 4.0.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::data_target::CLRDATA_ADDRESS;
use super::xclr_data_process::IXCLRDataModule;

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

    pub unsafe fn EnumerateErrors(
        &self,
        ppEnum: *mut *mut ISOSStackRefErrorEnum,
    ) -> HRESULT;
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

// Forward declarations for DAC structures (opaque pointers)
// These would need full definitions if you want to use them
pub type DacpThreadStoreData = std::ffi::c_void;
pub type DacpAppDomainStoreData = std::ffi::c_void;
pub type DacpAppDomainData = std::ffi::c_void;
pub type DacpAssemblyData = std::ffi::c_void;
pub type DacpModuleData = std::ffi::c_void;
pub type DacpThreadData = std::ffi::c_void;
pub type DacpMethodDescData = std::ffi::c_void;
pub type DacpReJitData = std::ffi::c_void;
pub type DacpCodeHeaderData = std::ffi::c_void;
pub type DacpJitManagerInfo = std::ffi::c_void;
pub type DacpThreadpoolData = std::ffi::c_void;
pub type DacpWorkRequestData = std::ffi::c_void;
pub type DacpHillClimbingLogEntry = std::ffi::c_void;
pub type DacpObjectData = std::ffi::c_void;
pub type DacpMethodTableData = std::ffi::c_void;
pub type DacpMethodTableFieldData = std::ffi::c_void;
pub type DacpMethodTableTransparencyData = std::ffi::c_void;
pub type DacpFieldDescData = std::ffi::c_void;
pub type DacpGcHeapData = std::ffi::c_void;
pub type DacpGcHeapDetails = std::ffi::c_void;
pub type DacpHeapSegmentData = std::ffi::c_void;
pub type DacpOomData = std::ffi::c_void;
pub type DacpGcHeapAnalyzeData = std::ffi::c_void;
pub type DacpDomainLocalModuleData = std::ffi::c_void;
pub type DacpThreadLocalModuleData = std::ffi::c_void;
pub type DacpSyncBlockData = std::ffi::c_void;
pub type DacpSyncBlockCleanupData = std::ffi::c_void;
pub type DacpRCWData = std::ffi::c_void;
pub type DacpCCWData = std::ffi::c_void;
pub type DacpCOMInterfacePointerData = std::ffi::c_void;
pub type DacpUsefulGlobalsData = std::ffi::c_void;
pub type DacpAllocData = std::ffi::c_void;
pub type DacpGenerationAllocData = std::ffi::c_void;
pub type DacpJitCodeHeapInfo = std::ffi::c_void;
pub type DacpMethodDescTransparencyData = std::ffi::c_void;
pub type DacpExceptionObjectData = std::ffi::c_void;
pub type DacpGCInterestingInfoData = std::ffi::c_void;
pub type DacpTieredVersionData = std::ffi::c_void;
pub type DacpMethodTableCollectibleData = std::ffi::c_void;
pub type DacpReJitData2 = std::ffi::c_void;
pub type DacpProfilerILData = std::ffi::c_void;
pub type DacpGenerationData = std::ffi::c_void;
pub type DACEHInfo = std::ffi::c_void;

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
pub type MODULEMAPTRAVERSE = Option<unsafe extern "system" fn(index: u32, methodTable: CLRDATA_ADDRESS, token: *mut std::ffi::c_void)>;

/// Callback for heap visiting.
pub type VISITHEAP = Option<unsafe extern "system" fn(blockData: CLRDATA_ADDRESS, blockSize: usize, blockIsCurrentBlock: i32)>;

/// Callback for RCW cleanup traversal.
pub type VISITRCWFORCLEANUP = Option<unsafe extern "system" fn(
    RCW: CLRDATA_ADDRESS,
    Context: CLRDATA_ADDRESS,
    Thread: CLRDATA_ADDRESS,
    bIsFreeThreaded: i32,
    token: *mut std::ffi::c_void,
) -> i32>;

/// Callback for exception handler info dumping.
pub type DUMPEHINFO = Option<unsafe extern "system" fn(
    clauseIndex: u32,
    totalClauses: u32,
    pEHInfo: *mut DACEHInfo,
    token: *mut std::ffi::c_void,
) -> i32>;

/// ISOSDacInterface - Main SOS debugging interface.
/// Provides methods for querying CLR runtime internal data.
#[interface("436f00f2-b42a-4b9f-870c-e73db66ae930")]
pub unsafe trait ISOSDacInterface: IUnknown {
    // ThreadStore
    pub unsafe fn GetThreadStoreData(&self, data: *mut DacpThreadStoreData) -> HRESULT;

    // AppDomains
    pub unsafe fn GetAppDomainStoreData(&self, data: *mut DacpAppDomainStoreData) -> HRESULT;
    pub unsafe fn GetAppDomainList(&self, count: u32, values: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetAppDomainData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpAppDomainData) -> HRESULT;
    pub unsafe fn GetAppDomainName(&self, addr: CLRDATA_ADDRESS, count: u32, name: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetDomainFromContext(&self, context: CLRDATA_ADDRESS, domain: *mut CLRDATA_ADDRESS) -> HRESULT;

    // Assemblies
    pub unsafe fn GetAssemblyList(&self, appDomain: CLRDATA_ADDRESS, count: i32, values: *mut CLRDATA_ADDRESS, pNeeded: *mut i32) -> HRESULT;
    pub unsafe fn GetAssemblyData(&self, baseDomainPtr: CLRDATA_ADDRESS, assembly: CLRDATA_ADDRESS, data: *mut DacpAssemblyData) -> HRESULT;
    pub unsafe fn GetAssemblyName(&self, assembly: CLRDATA_ADDRESS, count: u32, name: *mut u16, pNeeded: *mut u32) -> HRESULT;

    // Modules
    pub unsafe fn GetModule(&self, addr: CLRDATA_ADDRESS, module: *mut *mut IXCLRDataModule) -> HRESULT;
    pub unsafe fn GetModuleData(&self, moduleAddr: CLRDATA_ADDRESS, data: *mut DacpModuleData) -> HRESULT;
    pub unsafe fn TraverseModuleMap(&self, mmt: ModuleMapType, moduleAddr: CLRDATA_ADDRESS, pCallback: MODULEMAPTRAVERSE, token: *mut std::ffi::c_void) -> HRESULT;
    pub unsafe fn GetAssemblyModuleList(&self, assembly: CLRDATA_ADDRESS, count: u32, modules: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetILForModule(&self, moduleAddr: CLRDATA_ADDRESS, rva: u32, il: *mut CLRDATA_ADDRESS) -> HRESULT;

    // Threads
    pub unsafe fn GetThreadData(&self, thread: CLRDATA_ADDRESS, data: *mut DacpThreadData) -> HRESULT;
    pub unsafe fn GetThreadFromThinlockID(&self, thinLockId: u32, pThread: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetStackLimits(&self, threadPtr: CLRDATA_ADDRESS, lower: *mut CLRDATA_ADDRESS, upper: *mut CLRDATA_ADDRESS, fp: *mut CLRDATA_ADDRESS) -> HRESULT;

    // MethodDescs
    pub unsafe fn GetMethodDescData(&self, methodDesc: CLRDATA_ADDRESS, ip: CLRDATA_ADDRESS, data: *mut DacpMethodDescData, cRevertedRejitVersions: u32, rgRevertedRejitData: *mut DacpReJitData, pcNeededRevertedRejitData: *mut u32) -> HRESULT;
    pub unsafe fn GetMethodDescPtrFromIP(&self, ip: CLRDATA_ADDRESS, ppMD: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetMethodDescName(&self, methodDesc: CLRDATA_ADDRESS, count: u32, name: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetMethodDescPtrFromFrame(&self, frameAddr: CLRDATA_ADDRESS, ppMD: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetMethodDescFromToken(&self, moduleAddr: CLRDATA_ADDRESS, token: u32, methodDesc: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetMethodDescTransparencyData(&self, methodDesc: CLRDATA_ADDRESS, data: *mut DacpMethodDescTransparencyData) -> HRESULT;

    // JIT Data
    pub unsafe fn GetCodeHeaderData(&self, ip: CLRDATA_ADDRESS, data: *mut DacpCodeHeaderData) -> HRESULT;
    pub unsafe fn GetJitManagerList(&self, count: u32, managers: *mut DacpJitManagerInfo, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetJitHelperFunctionName(&self, ip: CLRDATA_ADDRESS, count: u32, name: *mut u8, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetJumpThunkTarget(&self, ctx: *mut std::ffi::c_void, targetIP: *mut CLRDATA_ADDRESS, targetMD: *mut CLRDATA_ADDRESS) -> HRESULT;

    // ThreadPool
    pub unsafe fn GetThreadpoolData(&self, data: *mut DacpThreadpoolData) -> HRESULT;
    pub unsafe fn GetWorkRequestData(&self, addrWorkRequest: CLRDATA_ADDRESS, data: *mut DacpWorkRequestData) -> HRESULT;
    pub unsafe fn GetHillClimbingLogEntry(&self, addr: CLRDATA_ADDRESS, data: *mut DacpHillClimbingLogEntry) -> HRESULT;

    // Objects
    pub unsafe fn GetObjectData(&self, objAddr: CLRDATA_ADDRESS, data: *mut DacpObjectData) -> HRESULT;
    pub unsafe fn GetObjectStringData(&self, obj: CLRDATA_ADDRESS, count: u32, stringData: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetObjectClassName(&self, obj: CLRDATA_ADDRESS, count: u32, className: *mut u16, pNeeded: *mut u32) -> HRESULT;

    // MethodTable
    pub unsafe fn GetMethodTableName(&self, mt: CLRDATA_ADDRESS, count: u32, mtName: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetMethodTableData(&self, mt: CLRDATA_ADDRESS, data: *mut DacpMethodTableData) -> HRESULT;
    pub unsafe fn GetMethodTableSlot(&self, mt: CLRDATA_ADDRESS, slot: u32, value: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetMethodTableFieldData(&self, mt: CLRDATA_ADDRESS, data: *mut DacpMethodTableFieldData) -> HRESULT;
    pub unsafe fn GetMethodTableTransparencyData(&self, mt: CLRDATA_ADDRESS, data: *mut DacpMethodTableTransparencyData) -> HRESULT;

    // EEClass
    pub unsafe fn GetMethodTableForEEClass(&self, eeClass: CLRDATA_ADDRESS, value: *mut CLRDATA_ADDRESS) -> HRESULT;

    // FieldDesc
    pub unsafe fn GetFieldDescData(&self, fieldDesc: CLRDATA_ADDRESS, data: *mut DacpFieldDescData) -> HRESULT;

    // Frames
    pub unsafe fn GetFrameName(&self, vtable: CLRDATA_ADDRESS, count: u32, frameName: *mut u16, pNeeded: *mut u32) -> HRESULT;

    // PEFiles
    pub unsafe fn GetPEFileBase(&self, addr: CLRDATA_ADDRESS, base: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetPEFileName(&self, addr: CLRDATA_ADDRESS, count: u32, fileName: *mut u16, pNeeded: *mut u32) -> HRESULT;

    // GC
    pub unsafe fn GetGCHeapData(&self, data: *mut DacpGcHeapData) -> HRESULT;
    pub unsafe fn GetGCHeapList(&self, count: u32, heaps: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetGCHeapDetails(&self, heap: CLRDATA_ADDRESS, details: *mut DacpGcHeapDetails) -> HRESULT;
    pub unsafe fn GetGCHeapStaticData(&self, data: *mut DacpGcHeapDetails) -> HRESULT;
    pub unsafe fn GetHeapSegmentData(&self, seg: CLRDATA_ADDRESS, data: *mut DacpHeapSegmentData) -> HRESULT;
    pub unsafe fn GetOOMData(&self, oomAddr: CLRDATA_ADDRESS, data: *mut DacpOomData) -> HRESULT;
    pub unsafe fn GetOOMStaticData(&self, data: *mut DacpOomData) -> HRESULT;
    pub unsafe fn GetHeapAnalyzeData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpGcHeapAnalyzeData) -> HRESULT;
    pub unsafe fn GetHeapAnalyzeStaticData(&self, data: *mut DacpGcHeapAnalyzeData) -> HRESULT;

    // DomainLocal
    pub unsafe fn GetDomainLocalModuleData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpDomainLocalModuleData) -> HRESULT;
    pub unsafe fn GetDomainLocalModuleDataFromAppDomain(&self, appDomainAddr: CLRDATA_ADDRESS, moduleID: i32, data: *mut DacpDomainLocalModuleData) -> HRESULT;
    pub unsafe fn GetDomainLocalModuleDataFromModule(&self, moduleAddr: CLRDATA_ADDRESS, data: *mut DacpDomainLocalModuleData) -> HRESULT;

    // ThreadLocal
    pub unsafe fn GetThreadLocalModuleData(&self, thread: CLRDATA_ADDRESS, index: u32, data: *mut DacpThreadLocalModuleData) -> HRESULT;

    // SyncBlock
    pub unsafe fn GetSyncBlockData(&self, number: u32, data: *mut DacpSyncBlockData) -> HRESULT;
    pub unsafe fn GetSyncBlockCleanupData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpSyncBlockCleanupData) -> HRESULT;

    // Handles
    pub unsafe fn GetHandleEnum(&self, ppHandleEnum: *mut *mut ISOSHandleEnum) -> HRESULT;
    pub unsafe fn GetHandleEnumForTypes(&self, types: *const u32, count: u32, ppHandleEnum: *mut *mut ISOSHandleEnum) -> HRESULT;
    pub unsafe fn GetHandleEnumForGC(&self, generation: u32, ppHandleEnum: *mut *mut ISOSHandleEnum) -> HRESULT;

    // EH
    pub unsafe fn TraverseEHInfo(&self, ip: CLRDATA_ADDRESS, pCallback: DUMPEHINFO, token: *mut std::ffi::c_void) -> HRESULT;
    pub unsafe fn GetNestedExceptionData(&self, exception: CLRDATA_ADDRESS, exceptionObject: *mut CLRDATA_ADDRESS, nextNestedException: *mut CLRDATA_ADDRESS) -> HRESULT;

    // StressLog
    pub unsafe fn GetStressLogAddress(&self, stressLog: *mut CLRDATA_ADDRESS) -> HRESULT;

    // Heaps
    pub unsafe fn TraverseLoaderHeap(&self, loaderHeapAddr: CLRDATA_ADDRESS, pCallback: VISITHEAP) -> HRESULT;
    pub unsafe fn GetCodeHeapList(&self, jitManager: CLRDATA_ADDRESS, count: u32, codeHeaps: *mut DacpJitCodeHeapInfo, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn TraverseVirtCallStubHeap(&self, pAppDomain: CLRDATA_ADDRESS, heaptype: VCSHeapType, pCallback: VISITHEAP) -> HRESULT;

    // Other
    pub unsafe fn GetUsefulGlobals(&self, data: *mut DacpUsefulGlobalsData) -> HRESULT;
    pub unsafe fn GetClrWatsonBuckets(&self, thread: CLRDATA_ADDRESS, pGenericModeBlock: *mut std::ffi::c_void) -> HRESULT;
    pub unsafe fn GetTLSIndex(&self, pIndex: *mut u32) -> HRESULT;
    pub unsafe fn GetDacModuleHandle(&self, phModule: *mut *mut std::ffi::c_void) -> HRESULT;

    // COM
    pub unsafe fn GetRCWData(&self, addr: CLRDATA_ADDRESS, data: *mut DacpRCWData) -> HRESULT;
    pub unsafe fn GetRCWInterfaces(&self, rcw: CLRDATA_ADDRESS, count: u32, interfaces: *mut DacpCOMInterfacePointerData, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetCCWData(&self, ccw: CLRDATA_ADDRESS, data: *mut DacpCCWData) -> HRESULT;
    pub unsafe fn GetCCWInterfaces(&self, ccw: CLRDATA_ADDRESS, count: u32, interfaces: *mut DacpCOMInterfacePointerData, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn TraverseRCWCleanupList(&self, cleanupListPtr: CLRDATA_ADDRESS, pCallback: VISITRCWFORCLEANUP, token: *mut std::ffi::c_void) -> HRESULT;

    // GC Reference Functions
    pub unsafe fn GetStackReferences(&self, osThreadID: u32, ppEnum: *mut *mut ISOSStackRefEnum) -> HRESULT;
    pub unsafe fn GetRegisterName(&self, regName: i32, count: u32, buffer: *mut u16, pNeeded: *mut u32) -> HRESULT;

    pub unsafe fn GetThreadAllocData(&self, thread: CLRDATA_ADDRESS, data: *mut DacpAllocData) -> HRESULT;
    pub unsafe fn GetHeapAllocData(&self, count: u32, data: *mut DacpGenerationAllocData, pNeeded: *mut u32) -> HRESULT;

    // BindingDisplay
    pub unsafe fn GetFailedAssemblyList(&self, appDomain: CLRDATA_ADDRESS, count: i32, values: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetPrivateBinPaths(&self, appDomain: CLRDATA_ADDRESS, count: i32, paths: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetAssemblyLocation(&self, assembly: CLRDATA_ADDRESS, count: i32, location: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetAppDomainConfigFile(&self, appDomain: CLRDATA_ADDRESS, count: i32, configFile: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetApplicationBase(&self, appDomain: CLRDATA_ADDRESS, count: i32, base: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetFailedAssemblyData(&self, assembly: CLRDATA_ADDRESS, pContext: *mut u32, pResult: *mut HRESULT) -> HRESULT;
    pub unsafe fn GetFailedAssemblyLocation(&self, assembly: CLRDATA_ADDRESS, count: u32, location: *mut u16, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetFailedAssemblyDisplayName(&self, assembly: CLRDATA_ADDRESS, count: u32, name: *mut u16, pNeeded: *mut u32) -> HRESULT;
}

/// ISOSDacInterface2 - Extended SOS debugging interface.
#[interface("A16026EC-96F4-40BA-87FB-5575986FB7AF")]
pub unsafe trait ISOSDacInterface2: IUnknown {
    pub unsafe fn GetObjectExceptionData(&self, objAddr: CLRDATA_ADDRESS, data: *mut DacpExceptionObjectData) -> HRESULT;
    pub unsafe fn IsRCWDCOMProxy(&self, rcwAddr: CLRDATA_ADDRESS, isDCOMProxy: *mut i32) -> HRESULT;
}

/// ISOSDacInterface3 - GC interesting info interface.
#[interface("B08C5CDC-FD8A-49C5-AB38-5FEEF35235B4")]
pub unsafe trait ISOSDacInterface3: IUnknown {
    pub unsafe fn GetGCInterestingInfoData(&self, interestingInfoAddr: CLRDATA_ADDRESS, data: *mut DacpGCInterestingInfoData) -> HRESULT;
    pub unsafe fn GetGCInterestingInfoStaticData(&self, data: *mut DacpGCInterestingInfoData) -> HRESULT;
    pub unsafe fn GetGCGlobalMechanisms(&self, globalMechanisms: *mut usize) -> HRESULT;
}

/// ISOSDacInterface4 - CLR notification interface.
#[interface("74B9D34C-A612-4B07-93DD-5462178FCE11")]
pub unsafe trait ISOSDacInterface4: IUnknown {
    pub unsafe fn GetClrNotification(&self, arguments: *mut CLRDATA_ADDRESS, count: i32, pNeeded: *mut i32) -> HRESULT;
}

/// ISOSDacInterface5 - Tiered compilation interface.
#[interface("127d6abe-6c86-4e48-8e7b-220781c58101")]
pub unsafe trait ISOSDacInterface5: IUnknown {
    pub unsafe fn GetTieredVersions(&self, methodDesc: CLRDATA_ADDRESS, rejitId: i32, nativeCodeAddrs: *mut DacpTieredVersionData, cNativeCodeAddrs: i32, pcNativeCodeAddrs: *mut i32) -> HRESULT;
}

/// ISOSDacInterface6 - Collectible method table interface.
#[interface("11206399-4B66-4EDB-98EA-85654E59AD45")]
pub unsafe trait ISOSDacInterface6: IUnknown {
    pub unsafe fn GetMethodTableCollectibleData(&self, mt: CLRDATA_ADDRESS, data: *mut DacpMethodTableCollectibleData) -> HRESULT;
}

/// ISOSDacInterface7 - ReJIT and profiler interface.
#[interface("c1020dde-fe98-4536-a53b-f35a74c327eb")]
pub unsafe trait ISOSDacInterface7: IUnknown {
    pub unsafe fn GetPendingReJITID(&self, methodDesc: CLRDATA_ADDRESS, pRejitId: *mut i32) -> HRESULT;
    pub unsafe fn GetReJITInformation(&self, methodDesc: CLRDATA_ADDRESS, rejitId: i32, pRejitData: *mut DacpReJitData2) -> HRESULT;
    pub unsafe fn GetProfilerModifiedILInformation(&self, methodDesc: CLRDATA_ADDRESS, pILData: *mut DacpProfilerILData) -> HRESULT;
    pub unsafe fn GetMethodsWithProfilerModifiedIL(&self, module: CLRDATA_ADDRESS, methodDescs: *mut CLRDATA_ADDRESS, cMethodDescs: i32, pcMethodDescs: *mut i32) -> HRESULT;
}

/// ISOSDacInterface8 - Generation and finalization interface.
#[interface("c12f35a9-e55c-4520-a894-b3dc5165dfce")]
pub unsafe trait ISOSDacInterface8: IUnknown {
    pub unsafe fn GetNumberGenerations(&self, pGenerations: *mut u32) -> HRESULT;
    pub unsafe fn GetGenerationTable(&self, cGenerations: u32, pGenerationData: *mut DacpGenerationData, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetFinalizationFillPointers(&self, cFillPointers: u32, pFinalizationFillPointers: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetGenerationTableSvr(&self, heapAddr: CLRDATA_ADDRESS, cGenerations: u32, pGenerationData: *mut DacpGenerationData, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetFinalizationFillPointersSvr(&self, heapAddr: CLRDATA_ADDRESS, cFillPointers: u32, pFinalizationFillPointers: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn GetAssemblyLoadContext(&self, methodTable: CLRDATA_ADDRESS, assemblyLoadContext: *mut CLRDATA_ADDRESS) -> HRESULT;
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
    pub unsafe fn GetObjectComWrappersData(&self, objAddr: CLRDATA_ADDRESS, rcw: *mut CLRDATA_ADDRESS, count: u32, mowList: *mut CLRDATA_ADDRESS, pNeeded: *mut u32) -> HRESULT;
    pub unsafe fn IsComWrappersCCW(&self, ccw: CLRDATA_ADDRESS, isComWrappersCCW: *mut i32) -> HRESULT;
    pub unsafe fn GetComWrappersCCWData(&self, ccw: CLRDATA_ADDRESS, managedObject: *mut CLRDATA_ADDRESS, refCount: *mut i32) -> HRESULT;
    pub unsafe fn IsComWrappersRCW(&self, rcw: CLRDATA_ADDRESS, isComWrappersRCW: *mut i32) -> HRESULT;
    pub unsafe fn GetComWrappersRCWData(&self, rcw: CLRDATA_ADDRESS, identity: *mut CLRDATA_ADDRESS) -> HRESULT;
}

/// ISOSDacInterface11 - Tagged memory interface.
#[interface("96BA1DB9-14CD-4492-8065-1CAAECF6E5CF")]
pub unsafe trait ISOSDacInterface11: IUnknown {
    pub unsafe fn IsTrackedType(&self, objAddr: CLRDATA_ADDRESS, isTrackedType: *mut i32, hasTaggedMemory: *mut i32) -> HRESULT;
    pub unsafe fn GetTaggedMemory(&self, objAddr: CLRDATA_ADDRESS, taggedMemory: *mut CLRDATA_ADDRESS, taggedMemorySizeInBytes: *mut usize) -> HRESULT;
}

/// ISOSDacInterface12 - Global allocation context interface.
#[interface("1b93bacc-8ca4-432d-943a-3e6e7ec0b0a3")]
pub unsafe trait ISOSDacInterface12: IUnknown {
    pub unsafe fn GetGlobalAllocationContext(&self, allocPtr: *mut CLRDATA_ADDRESS, allocLimit: *mut CLRDATA_ADDRESS) -> HRESULT;
}

/// ISOSDacInterface13 - Extended loader heap and memory region interface.
#[interface("3176a8ed-597b-4f54-a71f-83695c6a8c5e")]
pub unsafe trait ISOSDacInterface13: IUnknown {
    pub unsafe fn TraverseLoaderHeap(&self, loaderHeapAddr: CLRDATA_ADDRESS, kind: LoaderHeapKind, pCallback: VISITHEAP) -> HRESULT;
    pub unsafe fn GetDomainLoaderAllocator(&self, domainAddress: CLRDATA_ADDRESS, pLoaderAllocator: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetLoaderAllocatorHeapNames(&self, count: i32, ppNames: *mut *const i8, pNeeded: *mut i32) -> HRESULT;
    pub unsafe fn GetLoaderAllocatorHeaps(&self, loaderAllocator: CLRDATA_ADDRESS, count: i32, pLoaderHeaps: *mut CLRDATA_ADDRESS, pKinds: *mut LoaderHeapKind, pNeeded: *mut i32) -> HRESULT;
    pub unsafe fn GetHandleTableMemoryRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum) -> HRESULT;
    pub unsafe fn GetGCBookkeepingMemoryRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum) -> HRESULT;
    pub unsafe fn GetGCFreeRegions(&self, ppEnum: *mut *mut ISOSMemoryEnum) -> HRESULT;
    pub unsafe fn LockedFlush(&self) -> HRESULT;
}

/// ISOSDacInterface14 - Static base address interface.
#[interface("9aa22aca-6dc6-4a0c-b4e0-70d2416b9837")]
pub unsafe trait ISOSDacInterface14: IUnknown {
    pub unsafe fn GetStaticBaseAddress(&self, methodTable: CLRDATA_ADDRESS, nonGCStaticsAddress: *mut CLRDATA_ADDRESS, GCStaticsAddress: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetThreadStaticBaseAddress(&self, methodTable: CLRDATA_ADDRESS, thread: CLRDATA_ADDRESS, nonGCStaticsAddress: *mut CLRDATA_ADDRESS, GCStaticsAddress: *mut CLRDATA_ADDRESS) -> HRESULT;
    pub unsafe fn GetMethodTableInitializationFlags(&self, methodTable: CLRDATA_ADDRESS, initializationStatus: *mut MethodTableInitializationFlags) -> HRESULT;
}

/// ISOSDacInterface15 - Method table slot enumerator interface.
#[interface("7ed81261-52a9-4a23-a358-c3313dea30a8")]
pub unsafe trait ISOSDacInterface15: IUnknown {
    pub unsafe fn GetMethodTableSlotEnumerator(&self, mt: CLRDATA_ADDRESS, enumerator: *mut *mut ISOSMethodEnum) -> HRESULT;
}

/// ISOSDacInterface16 - GC dynamic adaptation mode interface.
#[interface("4ba12ff8-daac-4e43-ac56-98cf8d5c595d")]
pub unsafe trait ISOSDacInterface16: IUnknown {
    pub unsafe fn GetGCDynamicAdaptationMode(&self, pDynamicAdaptationMode: *mut i32) -> HRESULT;
}

