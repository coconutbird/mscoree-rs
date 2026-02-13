//! ICorProfiler interface definitions for .NET profiling.
//!
//! These interfaces are used by profiler DLLs to monitor and analyze .NET applications.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// COR_PRF_MONITOR flags - Events that can be monitored by a profiler.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CorPrfMonitor {
    None = 0,
    FunctionUnloads = 0x1,
    ClassLoads = 0x2,
    ModuleLoads = 0x4,
    AssemblyLoads = 0x8,
    AppDomainLoads = 0x10,
    JitCompilation = 0x20,
    Exceptions = 0x40,
    Clr = 0x80,
    Gc = 0x100,
    ObjectAllocated = 0x200,
    Threads = 0x400,
    Remoting = 0x800,
    Code = 0x1000,
    EnterLeave = 0x2000,
    Ccw = 0x4000,
    RemotingAsync = 0x8000,
    Inproc = 0x10000,
    Suspends = 0x20000,
    CacheSearches = 0x40000,
    All = 0x0007FFFF,
}

/// COR_PRF_GC_GENERATION - GC generation values.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CorPrfGcGeneration {
    Gen0 = 0,
    Gen1 = 1,
    Gen2 = 2,
    LargeObjectHeap = 3,
    PinnedObjectHeap = 4,
}

/// ICorProfilerCallback - Callback interface for profiler events.
#[interface("176FBED1-A55C-4796-98CA-A9DA0EF883E7")]
pub unsafe trait ICorProfilerCallback: IUnknown {
    /// Called when profiling is being initialized.
    pub unsafe fn Initialize(&self, pICorProfilerInfoUnk: *mut IUnknown) -> HRESULT;

    /// Called when profiling is shutting down.
    pub unsafe fn Shutdown(&self) -> HRESULT;

    /// Called when an AppDomain creation starts.
    pub unsafe fn AppDomainCreationStarted(&self, appDomainId: usize) -> HRESULT;

    /// Called when an AppDomain creation finishes.
    pub unsafe fn AppDomainCreationFinished(&self, appDomainId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when an AppDomain shutdown starts.
    pub unsafe fn AppDomainShutdownStarted(&self, appDomainId: usize) -> HRESULT;

    /// Called when an AppDomain shutdown finishes.
    pub unsafe fn AppDomainShutdownFinished(&self, appDomainId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when an assembly load starts.
    pub unsafe fn AssemblyLoadStarted(&self, assemblyId: usize) -> HRESULT;

    /// Called when an assembly load finishes.
    pub unsafe fn AssemblyLoadFinished(&self, assemblyId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when an assembly unload starts.
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyId: usize) -> HRESULT;

    /// Called when an assembly unload finishes.
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when a module load starts.
    pub unsafe fn ModuleLoadStarted(&self, moduleId: usize) -> HRESULT;

    /// Called when a module load finishes.
    pub unsafe fn ModuleLoadFinished(&self, moduleId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when a module unload starts.
    pub unsafe fn ModuleUnloadStarted(&self, moduleId: usize) -> HRESULT;

    /// Called when a module unload finishes.
    pub unsafe fn ModuleUnloadFinished(&self, moduleId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when a module is attached to an assembly.
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleId: usize, assemblyId: usize) -> HRESULT;

    /// Called when a class load starts.
    pub unsafe fn ClassLoadStarted(&self, classId: usize) -> HRESULT;

    /// Called when a class load finishes.
    pub unsafe fn ClassLoadFinished(&self, classId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when a class unload starts.
    pub unsafe fn ClassUnloadStarted(&self, classId: usize) -> HRESULT;

    /// Called when a class unload finishes.
    pub unsafe fn ClassUnloadFinished(&self, classId: usize, hrStatus: HRESULT) -> HRESULT;

    /// Called when a function is being unloaded.
    pub unsafe fn FunctionUnloadStarted(&self, functionId: usize) -> HRESULT;

    /// Called when JIT compilation starts.
    pub unsafe fn JITCompilationStarted(&self, functionId: usize, fIsSafeToBlock: i32) -> HRESULT;

    /// Called when JIT compilation finishes.
    pub unsafe fn JITCompilationFinished(&self, functionId: usize, hrStatus: HRESULT, fIsSafeToBlock: i32) -> HRESULT;

    /// Called when JIT caching starts.
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionId: usize, pbUseCachedFunction: *mut i32) -> HRESULT;

    /// Called when JIT caching finishes.
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionId: usize, result: u32) -> HRESULT;

    /// Called when JIT inlining starts.
    pub unsafe fn JITInlining(&self, callerId: usize, calleeId: usize, pfShouldInline: *mut i32) -> HRESULT;

    /// Called when a thread is created.
    pub unsafe fn ThreadCreated(&self, threadId: usize) -> HRESULT;

    /// Called when a thread is destroyed.
    pub unsafe fn ThreadDestroyed(&self, threadId: usize) -> HRESULT;

    /// Called when a thread is assigned an OS thread ID.
    pub unsafe fn ThreadAssignedToOSThread(&self, managedThreadId: usize, osThreadId: u32) -> HRESULT;

    /// Called when a remoting client invokes.
    pub unsafe fn RemotingClientInvocationStarted(&self) -> HRESULT;

    /// Called when remoting client is sending.
    pub unsafe fn RemotingClientSendingMessage(&self, pCookie: *const GUID, fIsAsync: i32) -> HRESULT;

    /// Called when remoting client receives.
    pub unsafe fn RemotingClientReceivingReply(&self, pCookie: *const GUID, fIsAsync: i32) -> HRESULT;

    /// Called when remoting client invocation finishes.
    pub unsafe fn RemotingClientInvocationFinished(&self) -> HRESULT;

    /// Called when remoting server receives.
    pub unsafe fn RemotingServerReceivingMessage(&self, pCookie: *const GUID, fIsAsync: i32) -> HRESULT;

    /// Called when remoting server invocation starts.
    pub unsafe fn RemotingServerInvocationStarted(&self) -> HRESULT;

    /// Called when remoting server invocation returns.
    pub unsafe fn RemotingServerInvocationReturned(&self) -> HRESULT;

    /// Called when remoting server sends.
    pub unsafe fn RemotingServerSendingReply(&self, pCookie: *const GUID, fIsAsync: i32) -> HRESULT;

    /// Called when an unmanaged-to-managed transition occurs.
    pub unsafe fn UnmanagedToManagedTransition(&self, functionId: usize, reason: u32) -> HRESULT;

    /// Called when a managed-to-unmanaged transition occurs.
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionId: usize, reason: u32) -> HRESULT;

    /// Called when runtime suspension starts.
    pub unsafe fn RuntimeSuspendStarted(&self, suspendReason: u32) -> HRESULT;

    /// Called when runtime suspension finishes.
    pub unsafe fn RuntimeSuspendFinished(&self) -> HRESULT;

    /// Called when runtime suspension aborts.
    pub unsafe fn RuntimeSuspendAborted(&self) -> HRESULT;

    /// Called when runtime resume starts.
    pub unsafe fn RuntimeResumeStarted(&self) -> HRESULT;

    /// Called when runtime resume finishes.
    pub unsafe fn RuntimeResumeFinished(&self) -> HRESULT;

    /// Called when a thread is suspended.
    pub unsafe fn RuntimeThreadSuspended(&self, threadId: usize) -> HRESULT;

    /// Called when a thread is resumed.
    pub unsafe fn RuntimeThreadResumed(&self, threadId: usize) -> HRESULT;

    /// Called when objects are moved in memory.
    pub unsafe fn MovedReferences(
        &self,
        cMovedObjectIDRanges: u32,
        oldObjectIDRangeStart: *const usize,
        newObjectIDRangeStart: *const usize,
        cObjectIDRangeLength: *const u32,
    ) -> HRESULT;

    /// Called when an object is allocated.
    pub unsafe fn ObjectAllocated(&self, objectId: usize, classId: usize) -> HRESULT;

    /// Called when objects are allocated.
    pub unsafe fn ObjectsAllocatedByClass(
        &self,
        cClassCount: u32,
        classIds: *const usize,
        cObjects: *const u32,
    ) -> HRESULT;

    /// Called when object references are collected.
    pub unsafe fn ObjectReferences(
        &self,
        objectId: usize,
        classId: usize,
        cObjectRefs: u32,
        objectRefIds: *const usize,
    ) -> HRESULT;

    /// Called when a root reference is collected.
    pub unsafe fn RootReferences(
        &self,
        cRootRefs: u32,
        rootRefIds: *const usize,
    ) -> HRESULT;

    /// Called when an exception is thrown.
    pub unsafe fn ExceptionThrown(&self, thrownObjectId: usize) -> HRESULT;

    /// Called when exception search starts.
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionId: usize) -> HRESULT;

    /// Called when exception search leaves.
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> HRESULT;

    /// Called when exception search filter enters.
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionId: usize) -> HRESULT;

    /// Called when exception search filter leaves.
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> HRESULT;

    /// Called when exception search catch starts.
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionId: usize) -> HRESULT;

    /// Called when a COM object reference is created.
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> HRESULT;

    /// Called when OS exception handler leaves.
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> HRESULT;

    /// Called when exception unwind function enters.
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionId: usize) -> HRESULT;

    /// Called when exception unwind function leaves.
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> HRESULT;

    /// Called when exception unwind finally enters.
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionId: usize) -> HRESULT;

    /// Called when exception unwind finally leaves.
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> HRESULT;

    /// Called when exception catch enters.
    pub unsafe fn ExceptionCatcherEnter(&self, functionId: usize, objectId: usize) -> HRESULT;

    /// Called when exception catch leaves.
    pub unsafe fn ExceptionCatcherLeave(&self) -> HRESULT;

    /// Called when a COM class info is requested.
    pub unsafe fn COMClassicVTableCreated(&self, wrappedClassId: usize, implementedIID: *const GUID, pVTable: *const c_void, cSlots: u32) -> HRESULT;

    /// Called when a COM class is destroyed.
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedClassId: usize, implementedIID: *const GUID, pVTable: *const c_void) -> HRESULT;

    /// Called when exception CLR catcher enters.
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> HRESULT;

    /// Called when exception CLR catcher executes.
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> HRESULT;
}

