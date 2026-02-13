//! ICorDebugManagedCallback interface definitions.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugStepReason - Reason for a step complete
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugStepReason {
    STEP_NORMAL = 0,
    STEP_RETURN = 1,
    STEP_CALL = 2,
    STEP_EXCEPTION_FILTER = 3,
    STEP_EXCEPTION_HANDLER = 4,
    STEP_INTERCEPT = 5,
    STEP_EXIT = 6,
}

/// CorDebugExceptionCallbackType - Type of exception callback
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugExceptionCallbackType {
    DEBUG_EXCEPTION_FIRST_CHANCE = 1,
    DEBUG_EXCEPTION_USER_FIRST_CHANCE = 2,
    DEBUG_EXCEPTION_CATCH_HANDLER_FOUND = 3,
    DEBUG_EXCEPTION_UNHANDLED = 4,
}

/// ICorDebugManagedCallback - Callback interface for debug events.
#[interface("3D6F5F60-7538-11D3-8D5B-00104B35E7EF")]
pub unsafe trait ICorDebugManagedCallback: IUnknown {
    /// Called when a breakpoint is hit.
    pub unsafe fn Breakpoint(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pBreakpoint: *mut IUnknown) -> HRESULT;

    /// Called when a stepper completes.
    pub unsafe fn StepComplete(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pStepper: *mut IUnknown, reason: u32) -> HRESULT;

    /// Called when execution breaks.
    pub unsafe fn Break(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown) -> HRESULT;

    /// Called when an exception is thrown.
    pub unsafe fn Exception(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, unhandled: i32) -> HRESULT;

    /// Called when an eval completes.
    pub unsafe fn EvalComplete(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pEval: *mut IUnknown) -> HRESULT;

    /// Called when an eval throws an exception.
    pub unsafe fn EvalException(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pEval: *mut IUnknown) -> HRESULT;

    /// Called when a process is created.
    pub unsafe fn CreateProcess(&self, pProcess: *mut IUnknown) -> HRESULT;

    /// Called when a process exits.
    pub unsafe fn ExitProcess(&self, pProcess: *mut IUnknown) -> HRESULT;

    /// Called when a thread is created.
    pub unsafe fn CreateThread(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown) -> HRESULT;

    /// Called when a thread exits.
    pub unsafe fn ExitThread(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown) -> HRESULT;

    /// Called when a module is loaded.
    pub unsafe fn LoadModule(&self, pAppDomain: *mut IUnknown, pModule: *mut IUnknown) -> HRESULT;

    /// Called when a module is unloaded.
    pub unsafe fn UnloadModule(&self, pAppDomain: *mut IUnknown, pModule: *mut IUnknown) -> HRESULT;

    /// Called when a class is loaded.
    pub unsafe fn LoadClass(&self, pAppDomain: *mut IUnknown, c: *mut IUnknown) -> HRESULT;

    /// Called when a class is unloaded.
    pub unsafe fn UnloadClass(&self, pAppDomain: *mut IUnknown, c: *mut IUnknown) -> HRESULT;

    /// Called when a debugger error occurs.
    pub unsafe fn DebuggerError(&self, pProcess: *mut IUnknown, errorHR: HRESULT, errorCode: u32) -> HRESULT;

    /// Called for log messages.
    pub unsafe fn LogMessage(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, lLevel: i32, pLogSwitchName: *const u16, pMessage: *const u16) -> HRESULT;

    /// Called when a log switch changes.
    pub unsafe fn LogSwitch(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, lLevel: i32, ulReason: u32, pLogSwitchName: *const u16, pParentName: *const u16) -> HRESULT;

    /// Called when an app domain is created.
    pub unsafe fn CreateAppDomain(&self, pProcess: *mut IUnknown, pAppDomain: *mut IUnknown) -> HRESULT;

    /// Called when an app domain exits.
    pub unsafe fn ExitAppDomain(&self, pProcess: *mut IUnknown, pAppDomain: *mut IUnknown) -> HRESULT;

    /// Called when an assembly is loaded.
    pub unsafe fn LoadAssembly(&self, pAppDomain: *mut IUnknown, pAssembly: *mut IUnknown) -> HRESULT;

    /// Called when an assembly is unloaded.
    pub unsafe fn UnloadAssembly(&self, pAppDomain: *mut IUnknown, pAssembly: *mut IUnknown) -> HRESULT;

    /// Called when control-C is pressed.
    pub unsafe fn ControlCTrap(&self, pProcess: *mut IUnknown) -> HRESULT;

    /// Called when a name changes.
    pub unsafe fn NameChange(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown) -> HRESULT;

    /// Called for update module symbols.
    pub unsafe fn UpdateModuleSymbols(&self, pAppDomain: *mut IUnknown, pModule: *mut IUnknown, pSymbolStream: *mut IUnknown) -> HRESULT;

    /// Called for edit and continue remap.
    pub unsafe fn EditAndContinueRemap(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pFunction: *mut IUnknown, fAccurate: i32) -> HRESULT;

    /// Called when a breakpoint set error occurs.
    pub unsafe fn BreakpointSetError(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pBreakpoint: *mut IUnknown, dwError: u32) -> HRESULT;
}

/// ICorDebugManagedCallback2 - Extended callback interface.
#[interface("250E5EEA-DB5C-4C76-B6F3-8C46F12E3203")]
pub unsafe trait ICorDebugManagedCallback2: IUnknown {
    /// Called for function remap opportunity.
    pub unsafe fn FunctionRemapOpportunity(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pOldFunction: *mut IUnknown, pNewFunction: *mut IUnknown, oldILOffset: u32) -> HRESULT;

    /// Called when a connection is created.
    pub unsafe fn CreateConnection(&self, pProcess: *mut IUnknown, dwConnectionId: u32, pConnName: *const u16) -> HRESULT;

    /// Called when a connection changes.
    pub unsafe fn ChangeConnection(&self, pProcess: *mut IUnknown, dwConnectionId: u32) -> HRESULT;

    /// Called when a connection is destroyed.
    pub unsafe fn DestroyConnection(&self, pProcess: *mut IUnknown, dwConnectionId: u32) -> HRESULT;

    /// Called for exception events.
    pub unsafe fn Exception(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pFrame: *mut IUnknown, nOffset: u32, dwEventType: u32, dwFlags: u32) -> HRESULT;

    /// Called for exception unwind events.
    pub unsafe fn ExceptionUnwind(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, dwEventType: u32, dwFlags: u32) -> HRESULT;

    /// Called for function remap complete.
    pub unsafe fn FunctionRemapComplete(&self, pAppDomain: *mut IUnknown, pThread: *mut IUnknown, pFunction: *mut IUnknown) -> HRESULT;

    /// Called for MDA (Managed Debugging Assistant) notifications.
    pub unsafe fn MDANotification(&self, pController: *mut IUnknown, pThread: *mut IUnknown, pMDA: *mut IUnknown) -> HRESULT;
}

/// ICorDebugManagedCallback3 - Callback interface for custom notifications.
#[interface("264EA0FC-2591-49AA-868E-835E6515323F")]
pub unsafe trait ICorDebugManagedCallback3: IUnknown {
    /// Called for custom debugger notifications.
    pub unsafe fn CustomNotification(&self, pThread: *mut IUnknown, pAppDomain: *mut IUnknown) -> HRESULT;
}

/// ICorDebugManagedCallback4 - Callback interface for .NET 5+.
#[interface("322911AE-16A5-49BA-84A3-ED69678138A3")]
pub unsafe trait ICorDebugManagedCallback4: IUnknown {
    /// Called before a GC starts.
    pub unsafe fn BeforeGarbageCollection(&self, pProcess: *mut IUnknown) -> HRESULT;

    /// Called after a GC completes.
    pub unsafe fn AfterGarbageCollection(&self, pProcess: *mut IUnknown) -> HRESULT;

    /// Called when data breakpoint is hit.
    pub unsafe fn DataBreakpoint(&self, pProcess: *mut IUnknown, pThread: *mut IUnknown, pContext: *const u8, contextSize: u32) -> HRESULT;
}

