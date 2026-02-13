//! ICorDebugProcess interface definitions.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugThreadState - State of a thread for debugging
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugThreadState {
    THREAD_RUN = 0,
    THREAD_SUSPEND = 1,
}

/// CorDebugStateChange - Type of state change
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugStateChange {
    PROCESS_RUNNING = 0x0001,
    FLUSH_ALL = 0x0002,
}

/// ICorDebugProcess - Represents a process being debugged.
#[interface("3D6F5F64-7538-11D3-8D5B-00104B35E7EF")]
pub unsafe trait ICorDebugProcess: IUnknown {
    // ICorDebugController methods
    pub unsafe fn Stop(&self, dwTimeoutIgnored: u32) -> HRESULT;
    pub unsafe fn Continue(&self, fIsOutOfBand: i32) -> HRESULT;
    pub unsafe fn IsRunning(&self, pbRunning: *mut i32) -> HRESULT;
    pub unsafe fn HasQueuedCallbacks(&self, pThread: *mut IUnknown, pbQueued: *mut i32) -> HRESULT;
    pub unsafe fn EnumerateThreads(&self, ppThreads: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn SetAllThreadsDebugState(&self, state: u32, pExceptThisThread: *mut IUnknown) -> HRESULT;
    pub unsafe fn Detach(&self) -> HRESULT;
    pub unsafe fn Terminate(&self, exitCode: u32) -> HRESULT;
    pub unsafe fn CanCommitChanges(&self, cSnapshots: u32, pSnapshots: *mut *mut IUnknown, pError: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn CommitChanges(&self, cSnapshots: u32, pSnapshots: *mut *mut IUnknown, pError: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugProcess methods
    /// Get the OS process ID.
    pub unsafe fn GetID(&self, pdwProcessId: *mut u32) -> HRESULT;

    /// Get the process handle.
    pub unsafe fn GetHandle(&self, phProcessHandle: *mut *mut c_void) -> HRESULT;

    /// Get a thread by OS thread ID.
    pub unsafe fn GetThread(&self, dwThreadId: u32, ppThread: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate all objects in the process.
    pub unsafe fn EnumerateObjects(&self, ppObjects: *mut *mut IUnknown) -> HRESULT;

    /// Check if an address is a transition stub.
    pub unsafe fn IsTransitionStub(&self, address: u64, pbTransitionStub: *mut i32) -> HRESULT;

    /// Check if the OS thread is owned by the runtime.
    pub unsafe fn IsOSSuspended(&self, threadID: u32, pbSuspended: *mut i32) -> HRESULT;

    /// Get the thread context.
    pub unsafe fn GetThreadContext(&self, threadID: u32, contextSize: u32, context: *mut u8) -> HRESULT;

    /// Set the thread context.
    pub unsafe fn SetThreadContext(&self, threadID: u32, contextSize: u32, context: *const u8) -> HRESULT;

    /// Read memory from the process.
    pub unsafe fn ReadMemory(&self, address: u64, size: u32, buffer: *mut u8, read: *mut u32) -> HRESULT;

    /// Write memory to the process.
    pub unsafe fn WriteMemory(&self, address: u64, size: u32, buffer: *const u8, written: *mut u32) -> HRESULT;

    /// Clear the current exception on a thread.
    pub unsafe fn ClearCurrentException(&self, threadID: u32) -> HRESULT;

    /// Enable logging for a thread.
    pub unsafe fn EnableLogMessages(&self, fOnOff: i32) -> HRESULT;

    /// Modify log switch settings.
    pub unsafe fn ModifyLogSwitch(&self, pLogSwitchName: *const u16, lLevel: i32) -> HRESULT;

    /// Enumerate all app domains.
    pub unsafe fn EnumerateAppDomains(&self, ppAppDomains: *mut *mut IUnknown) -> HRESULT;

    /// Get the object representing the process.
    pub unsafe fn GetObject(&self, ppObject: *mut *mut IUnknown) -> HRESULT;

    /// Get a thread by its CLR thread ID.
    pub unsafe fn ThreadForFiberCookie(&self, fiberCookie: u32, ppThread: *mut *mut IUnknown) -> HRESULT;

    /// Get the helper thread ID.
    pub unsafe fn GetHelperThreadID(&self, pThreadID: *mut u32) -> HRESULT;
}

/// ICorDebugProcess2 - Extended process debugging interface.
#[interface("AD1B3588-0EF0-4744-A496-AA09A9F80371")]
pub unsafe trait ICorDebugProcess2: IUnknown {
    /// Get the thread for a task ID.
    pub unsafe fn GetThreadForTaskID(&self, taskid: u64, ppThread: *mut *mut IUnknown) -> HRESULT;

    /// Get the runtime version.
    pub unsafe fn GetVersion(&self, version: *mut u64) -> HRESULT;

    /// Set unmanaged breakpoint.
    pub unsafe fn SetUnmanagedBreakpoint(&self, address: u64, bufsize: u32, buffer: *mut u8, bufLen: *mut u32) -> HRESULT;

    /// Clear unmanaged breakpoint.
    pub unsafe fn ClearUnmanagedBreakpoint(&self, address: u64) -> HRESULT;

    /// Set desired NGEN compiler flags.
    pub unsafe fn SetDesiredNGENCompilerFlags(&self, pdwFlags: u32) -> HRESULT;

    /// Get desired NGEN compiler flags.
    pub unsafe fn GetDesiredNGENCompilerFlags(&self, pdwFlags: *mut u32) -> HRESULT;

    /// Get a reference value for an object.
    pub unsafe fn GetReferenceValueFromGCHandle(&self, handle: u64, pOutValue: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugProcess4 - Process interface for exception handling.
#[interface("21E9D9C0-FCB8-11DF-8CFF-0800200C9A66")]
pub unsafe trait ICorDebugProcess4: IUnknown {
    /// Filter exceptions based on first/second chance.
    pub unsafe fn Filter(
        &self,
        pRecord: *const u8,
        recordSize: u32,
        contextRecord: *const u8,
        contextSize: u32,
        flags: u32,
        pContinueStatus: *mut u32,
    ) -> HRESULT;

    /// Process a state change.
    pub unsafe fn ProcessStateChanged(&self, change: u32) -> HRESULT;
}

/// ICorDebugProcess5 - Process interface for GC heap inspection.
#[interface("21E9D9C0-FCB8-11DF-8CFF-0800200C9A67")]
pub unsafe trait ICorDebugProcess5: IUnknown {
    /// Get the GC heap information.
    pub unsafe fn GetGCHeapInformation(&self, pHeapInfo: *mut c_void) -> HRESULT;

    /// Enumerate the GC heap.
    pub unsafe fn EnumerateHeap(&self, ppObjects: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate heap regions.
    pub unsafe fn EnumerateHeapRegions(&self, ppRegions: *mut *mut IUnknown) -> HRESULT;

    /// Get an object by address.
    pub unsafe fn GetObject(&self, addr: u64, pObject: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate GC references for an object.
    pub unsafe fn EnumerateGCReferences(&self, enumerateWeakReferences: i32, ppEnum: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate handles.
    pub unsafe fn EnumerateHandles(&self, types: *const u32, numTypes: u32, ppEnum: *mut *mut IUnknown) -> HRESULT;

    /// Get type ID for a type.
    pub unsafe fn GetTypeID(&self, obj: u64, pId: *mut c_void) -> HRESULT;

    /// Get type for a type ID.
    pub unsafe fn GetTypeForTypeID(&self, id: c_void, ppType: *mut *mut IUnknown) -> HRESULT;

    /// Get array layout.
    pub unsafe fn GetArrayLayout(&self, id: c_void, pLayout: *mut c_void) -> HRESULT;

    /// Get type layout.
    pub unsafe fn GetTypeLayout(&self, id: c_void, pLayout: *mut c_void) -> HRESULT;

    /// Get type fields.
    pub unsafe fn GetTypeFields(&self, id: c_void, celt: u32, fields: *mut c_void, pceltNeeded: *mut u32) -> HRESULT;

    /// Enable NGEN policy.
    pub unsafe fn EnableNGENPolicy(&self, ePolicy: u32) -> HRESULT;
}

/// ICorDebugProcess6 - Process interface for .NET Native debugging.
#[interface("11588775-7205-4CEB-A41A-93753C3153E9")]
pub unsafe trait ICorDebugProcess6: IUnknown {
    /// Decode an exception debug event.
    pub unsafe fn DecodeEvent(
        &self,
        pRecord: *const u8,
        countBytes: u32,
        format: u32,
        dwFlags: u32,
        dwThreadId: u32,
        pCallback: *mut IUnknown,
    ) -> HRESULT;

    /// Process state changed.
    pub unsafe fn ProcessStateChanged(&self, change: u32) -> HRESULT;

    /// Get code at an address.
    pub unsafe fn GetCode(&self, codeAddress: u64, ppCode: *mut *mut IUnknown) -> HRESULT;

    /// Enable virtual module splitting.
    pub unsafe fn EnableVirtualModuleSplitting(&self, enableSplitting: i32) -> HRESULT;

    /// Set marker for profiler heap.
    pub unsafe fn MarkDebuggerAttached(&self, fIsAttached: i32) -> HRESULT;

    /// Get the export step info.
    pub unsafe fn GetExportStepInfo(
        &self,
        pszExportName: *const u16,
        pInvokeKind: *mut u32,
        pInvokePurpose: *mut u32,
    ) -> HRESULT;
}

/// ICorDebugProcess7 - Process interface for in-memory modules.
#[interface("9B2C54E4-119F-4D6F-B402-527603266D69")]
pub unsafe trait ICorDebugProcess7: IUnknown {
    /// Set write watch for JIT debugging.
    pub unsafe fn SetWriteableMetadataUpdateMode(&self, flags: u32) -> HRESULT;
}

/// ICorDebugProcess8 - Process interface for exception callbacks.
#[interface("2E6F28C1-85EB-4141-80AD-0A90944B9639")]
pub unsafe trait ICorDebugProcess8: IUnknown {
    /// Enable exception callbacks outside of JMC code.
    pub unsafe fn EnableExceptionCallbacksOutsideOfMyCode(&self, enableExceptionsOutsideOfJMC: i32) -> HRESULT;
}

/// ICorDebugProcess10 - Process interface for .NET Core 3.0+.
#[interface("8F378F6F-1017-4461-9890-ECF64C54079F")]
pub unsafe trait ICorDebugProcess10: IUnknown {
    /// Enable GC notification events.
    pub unsafe fn EnableGCNotificationEvents(&self, fEnable: i32) -> HRESULT;
}

/// ICorDebugProcess11 - Process interface for .NET 5+.
#[interface("344B37AA-F2C0-4D3B-9909-91CCF787DA8C")]
pub unsafe trait ICorDebugProcess11: IUnknown {
    /// Enumerate loaded .NET modules.
    pub unsafe fn EnumerateLoaderHeapMemoryRegions(&self, ppEnum: *mut *mut IUnknown) -> HRESULT;
}
