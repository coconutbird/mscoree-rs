//! ICorDebugThread interface definitions.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugUserState - User state of a thread
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugUserState {
    USER_STOP_REQUESTED = 0x01,
    USER_SUSPEND_REQUESTED = 0x02,
    USER_BACKGROUND = 0x04,
    USER_UNSTARTED = 0x08,
    USER_STOPPED = 0x10,
    USER_WAIT_SLEEP_JOIN = 0x20,
    USER_SUSPENDED = 0x40,
    USER_UNSAFE_POINT = 0x80,
    USER_THREADPOOL = 0x100,
}

/// ICorDebugThread - Represents a managed thread.
#[interface("938C6D66-7FB6-4F69-B389-425B8987329B")]
pub unsafe trait ICorDebugThread: IUnknown {
    /// Get the process containing this thread.
    pub unsafe fn GetProcess(&self, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Get the OS thread ID.
    pub unsafe fn GetID(&self, pdwThreadId: *mut u32) -> HRESULT;

    /// Get the handle to the OS thread.
    pub unsafe fn GetHandle(&self, phThreadHandle: *mut *mut c_void) -> HRESULT;

    /// Get the app domain containing this thread.
    pub unsafe fn GetAppDomain(&self, ppAppDomain: *mut *mut IUnknown) -> HRESULT;

    /// Set the debug state of this thread.
    pub unsafe fn SetDebugState(&self, state: u32) -> HRESULT;

    /// Get the debug state of this thread.
    pub unsafe fn GetDebugState(&self, pState: *mut u32) -> HRESULT;

    /// Get the user state of this thread.
    pub unsafe fn GetUserState(&self, pState: *mut u32) -> HRESULT;

    /// Get the current exception on this thread.
    pub unsafe fn GetCurrentException(&self, ppExceptionObject: *mut *mut IUnknown) -> HRESULT;

    /// Clear the current exception.
    pub unsafe fn ClearCurrentException(&self) -> HRESULT;

    /// Create a stepper for this thread.
    pub unsafe fn CreateStepper(&self, ppStepper: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate all stack chains.
    pub unsafe fn EnumerateChains(&self, ppChains: *mut *mut IUnknown) -> HRESULT;

    /// Get the active chain.
    pub unsafe fn GetActiveChain(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Get the active frame.
    pub unsafe fn GetActiveFrame(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;

    /// Get the register set for this thread.
    pub unsafe fn GetRegisterSet(&self, ppRegisters: *mut *mut IUnknown) -> HRESULT;

    /// Create an eval object for this thread.
    pub unsafe fn CreateEval(&self, ppEval: *mut *mut IUnknown) -> HRESULT;

    /// Get the object representing this thread.
    pub unsafe fn GetObject(&self, ppObject: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugThread2 - Extended thread interface.
#[interface("2BD956D9-7B07-4BEF-8A98-12AA862417C5")]
pub unsafe trait ICorDebugThread2: IUnknown {
    /// Get the active functions on this thread.
    pub unsafe fn GetActiveFunctions(&self, cFunctions: u32, pcFunctions: *mut u32, pFunctions: *mut c_void) -> HRESULT;

    /// Get the connection ID.
    pub unsafe fn GetConnectionID(&self, pdwConnectionId: *mut u32) -> HRESULT;

    /// Get the task ID.
    pub unsafe fn GetTaskID(&self, pTaskId: *mut u64) -> HRESULT;

    /// Get the volatile OS thread ID.
    pub unsafe fn GetVolatileOSThreadID(&self, pdwTid: *mut u32) -> HRESULT;

    /// Intercept the current exception.
    pub unsafe fn InterceptCurrentException(&self, pFrame: *mut IUnknown) -> HRESULT;
}

/// ICorDebugThread3 - Thread interface for stack walking.
#[interface("F8544EC3-5E4E-46C7-8D3E-A52B8405B1F5")]
pub unsafe trait ICorDebugThread3: IUnknown {
    /// Create a stack walk object.
    pub unsafe fn CreateStackWalk(&self, ppStackWalk: *mut *mut IUnknown) -> HRESULT;

    /// Get the active internal frames.
    pub unsafe fn GetActiveInternalFrames(&self, cInternalFrames: u32, pcInternalFrames: *mut u32, ppInternalFrames: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugThread4 - Thread interface for blocking info.
#[interface("1A1F204B-1C66-4637-823F-3EE6C744A69C")]
pub unsafe trait ICorDebugThread4: IUnknown {
    /// Check if the thread has an unhandled exception.
    pub unsafe fn HasUnhandledException(&self) -> HRESULT;

    /// Get the blocking objects for this thread.
    pub unsafe fn GetBlockingObjects(&self, ppBlockingObjectEnum: *mut *mut IUnknown) -> HRESULT;

    /// Get the current custom debugger notification.
    pub unsafe fn GetCurrentCustomDebuggerNotification(&self, ppNotificationObject: *mut *mut IUnknown) -> HRESULT;
}

