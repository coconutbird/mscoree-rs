//! ICorDebug interface definitions - Core debugging interfaces.
//!
//! These interfaces are from mscordbi.dll and provide managed debugging capabilities.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugInterfaceVersion - Version of the debugging interface
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugInterfaceVersion {
    CorDebugInvalidVersion = 0,
    CorDebugVersion_1_0 = 1,
    CorDebugVersion_1_1 = 2,
    CorDebugVersion_2_0 = 3,
    CorDebugVersion_4_0 = 4,
    CorDebugVersion_4_5 = 5,
    // Note: CorDebugLatestVersion is an alias for the latest version (4_5)
}

/// CorDebugCreateProcessFlags
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugCreateProcessFlags {
    DEBUG_NO_SPECIAL_OPTIONS = 0,
}

/// ICorDebug - Main entry point for managed debugging.
///
/// This is the primary interface for debugging managed code.
/// Obtain an instance using CreateDebuggingInterfaceFromVersion or similar.
#[interface("3D6F5F61-7538-11D3-8D5B-00104B35E7EF")]
pub unsafe trait ICorDebug: IUnknown {
    /// Initialize the debugging services.
    pub unsafe fn Initialize(&self) -> HRESULT;

    /// Terminate the debugging services.
    pub unsafe fn Terminate(&self) -> HRESULT;

    /// Set the managed callback interface.
    pub unsafe fn SetManagedHandler(&self, pCallback: *mut IUnknown) -> HRESULT;

    /// Set the unmanaged callback interface.
    pub unsafe fn SetUnmanagedHandler(&self, pCallback: *mut IUnknown) -> HRESULT;

    /// Create a process for debugging.
    pub unsafe fn CreateProcess(
        &self,
        lpApplicationName: *const u16,
        lpCommandLine: *mut u16,
        lpProcessAttributes: *mut c_void,
        lpThreadAttributes: *mut c_void,
        bInheritHandles: i32,
        dwCreationFlags: u32,
        lpEnvironment: *mut c_void,
        lpCurrentDirectory: *const u16,
        lpStartupInfo: *mut c_void,
        lpProcessInformation: *mut c_void,
        debuggingFlags: u32,
        ppProcess: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Attach to an existing process.
    pub unsafe fn DebugActiveProcess(
        &self,
        id: u32,
        win32Attach: i32,
        ppProcess: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Enumerate all processes being debugged.
    pub unsafe fn EnumerateProcesses(&self, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Get a process by OS process ID.
    pub unsafe fn GetProcess(&self, dwProcessId: u32, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Check if a debugger can launch a process.
    pub unsafe fn CanLaunchOrAttach(&self, dwProcessId: u32, win32DebuggingEnabled: i32)
    -> HRESULT;
}

/// ICorDebug2 - Extended debugging interface.
#[interface("ECCCCF2E-B286-4B3E-A983-860A8793D105")]
pub unsafe trait ICorDebug2: IUnknown {
    // ICorDebug2 is currently empty in the CLR
}

/// ICorDebugController - Base interface for process/appdomain control.
#[interface("3D6F5F62-7538-11D3-8D5B-00104B35E7EF")]
pub unsafe trait ICorDebugController: IUnknown {
    /// Stop all managed execution in the process/appdomain.
    pub unsafe fn Stop(&self, dwTimeoutIgnored: u32) -> HRESULT;

    /// Continue execution after a debug event.
    pub unsafe fn Continue(&self, fIsOutOfBand: i32) -> HRESULT;

    /// Check if the process/appdomain is running.
    pub unsafe fn IsRunning(&self, pbRunning: *mut i32) -> HRESULT;

    /// Check if there are queued callbacks.
    pub unsafe fn HasQueuedCallbacks(&self, pThread: *mut IUnknown, pbQueued: *mut i32) -> HRESULT;

    /// Enumerate all threads.
    pub unsafe fn EnumerateThreads(&self, ppThreads: *mut *mut IUnknown) -> HRESULT;

    /// Set all threads to debug state.
    pub unsafe fn SetAllThreadsDebugState(
        &self,
        state: u32,
        pExceptThisThread: *mut IUnknown,
    ) -> HRESULT;

    /// Detach from the process/appdomain.
    pub unsafe fn Detach(&self) -> HRESULT;

    /// Terminate the process/appdomain.
    pub unsafe fn Terminate(&self, exitCode: u32) -> HRESULT;

    /// Request a callback when a managed exception occurs.
    pub unsafe fn CanCommitChanges(
        &self,
        cSnapshots: u32,
        pSnapshots: *mut *mut IUnknown,
        pError: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Commit changes from Edit and Continue.
    pub unsafe fn CommitChanges(
        &self,
        cSnapshots: u32,
        pSnapshots: *mut *mut IUnknown,
        pError: *mut *mut IUnknown,
    ) -> HRESULT;
}
