//! ICorDebugStepper, ICorDebugBreakpoint, and ICorDebugEval interface definitions.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugUnmappedStop - Unmapped stop options
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugUnmappedStop {
    STOP_NONE = 0x0,
    STOP_PROLOG = 0x01,
    STOP_EPILOG = 0x02,
    STOP_NO_MAPPING_INFO = 0x04,
    STOP_OTHER_UNMAPPED = 0x08,
    STOP_UNMANAGED = 0x10,
    STOP_ALL = 0xffff,
}

/// CorDebugIntercept - Intercept options
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugIntercept {
    INTERCEPT_NONE = 0x0,
    INTERCEPT_CLASS_INIT = 0x01,
    INTERCEPT_EXCEPTION_FILTER = 0x02,
    INTERCEPT_SECURITY = 0x04,
    INTERCEPT_CONTEXT_POLICY = 0x08,
    INTERCEPT_INTERCEPTION = 0x10,
    INTERCEPT_ALL = 0xffff,
}

/// ICorDebugStepper - Controls stepping through code.
#[interface("CC7BCAEC-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugStepper: IUnknown {
    /// Check if this stepper is active.
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;

    /// Deactivate this stepper.
    pub unsafe fn Deactivate(&self) -> HRESULT;

    /// Set intercept mask.
    pub unsafe fn SetInterceptMask(&self, mask: u32) -> HRESULT;

    /// Set unmapped stop mask.
    pub unsafe fn SetUnmappedStopMask(&self, mask: u32) -> HRESULT;

    /// Step into the next instruction.
    pub unsafe fn Step(&self, bStepIn: i32) -> HRESULT;

    /// Step over a range of code.
    pub unsafe fn StepRange(&self, bStepIn: i32, ranges: *const u32, cRangeCount: u32) -> HRESULT;

    /// Step out of the current function.
    pub unsafe fn StepOut(&self) -> HRESULT;

    /// Set range IL.
    pub unsafe fn SetRangeIL(&self, bIL: i32) -> HRESULT;
}

/// ICorDebugStepper2 - Extended stepper interface.
#[interface("C5B6E9C3-E7D1-4A8E-873B-7F047F0706F7")]
pub unsafe trait ICorDebugStepper2: IUnknown {
    /// Set JMC (Just My Code) mode.
    pub unsafe fn SetJMC(&self, fIsJMCStepper: i32) -> HRESULT;
}

/// ICorDebugBreakpoint - Represents a breakpoint.
#[interface("CC7BCAE8-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugBreakpoint: IUnknown {
    /// Activate or deactivate this breakpoint.
    pub unsafe fn Activate(&self, bActive: i32) -> HRESULT;

    /// Check if this breakpoint is active.
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;
}

/// ICorDebugFunctionBreakpoint - Breakpoint on a function.
#[interface("CC7BCAE9-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugFunctionBreakpoint: IUnknown {
    // ICorDebugBreakpoint methods
    pub unsafe fn Activate(&self, bActive: i32) -> HRESULT;
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;

    // ICorDebugFunctionBreakpoint methods
    /// Get the function for this breakpoint.
    pub unsafe fn GetFunction(&self, ppFunction: *mut *mut IUnknown) -> HRESULT;

    /// Get the offset in the function.
    pub unsafe fn GetOffset(&self, pnOffset: *mut u32) -> HRESULT;
}

/// ICorDebugModuleBreakpoint - Breakpoint on a module.
#[interface("CC7BCAEA-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugModuleBreakpoint: IUnknown {
    // ICorDebugBreakpoint methods
    pub unsafe fn Activate(&self, bActive: i32) -> HRESULT;
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;

    // ICorDebugModuleBreakpoint methods
    /// Get the module for this breakpoint.
    pub unsafe fn GetModule(&self, ppModule: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugValueBreakpoint - Breakpoint on a value.
#[interface("CC7BCAEB-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugValueBreakpoint: IUnknown {
    // ICorDebugBreakpoint methods
    pub unsafe fn Activate(&self, bActive: i32) -> HRESULT;
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;

    // ICorDebugValueBreakpoint methods
    /// Get the value for this breakpoint.
    pub unsafe fn GetValue(&self, ppValue: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugEval - Evaluates expressions in the debuggee.
#[interface("CC7BCAF6-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugEval: IUnknown {
    /// Call a function.
    pub unsafe fn CallFunction(&self, pFunction: *mut IUnknown, nArgs: u32, ppArgs: *mut *mut IUnknown) -> HRESULT;

    /// Create a new object.
    pub unsafe fn NewObject(&self, pConstructor: *mut IUnknown, nArgs: u32, ppArgs: *mut *mut IUnknown) -> HRESULT;

    /// Create a new object without calling constructor.
    pub unsafe fn NewObjectNoConstructor(&self, pClass: *mut IUnknown) -> HRESULT;

    /// Create a new string.
    pub unsafe fn NewString(&self, string: *const u16) -> HRESULT;

    /// Create a new array.
    pub unsafe fn NewArray(&self, elementType: u32, pElementClass: *mut IUnknown, rank: u32, dims: *const u32, lowBounds: *const u32) -> HRESULT;

    /// Check if eval is active.
    pub unsafe fn IsActive(&self, pbActive: *mut i32) -> HRESULT;

    /// Abort the eval.
    pub unsafe fn Abort(&self) -> HRESULT;

    /// Get the result of the eval.
    pub unsafe fn GetResult(&self, ppResult: *mut *mut IUnknown) -> HRESULT;

    /// Get the thread for this eval.
    pub unsafe fn GetThread(&self, ppThread: *mut *mut IUnknown) -> HRESULT;

    /// Create a value.
    pub unsafe fn CreateValue(&self, elementType: u32, pElementClass: *mut IUnknown, ppValue: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugEval2 - Extended eval interface.
#[interface("FB0D9CE7-BE66-4683-9D32-A42A04E2FD91")]
pub unsafe trait ICorDebugEval2: IUnknown {
    /// Call a function with type arguments.
    pub unsafe fn CallParameterizedFunction(&self, pFunction: *mut IUnknown, nTypeArgs: u32, ppTypeArgs: *mut *mut IUnknown, nArgs: u32, ppArgs: *mut *mut IUnknown) -> HRESULT;

    /// Create a value with type.
    pub unsafe fn CreateValueForType(&self, pType: *mut IUnknown, ppValue: *mut *mut IUnknown) -> HRESULT;

    /// Create a new object with type arguments.
    pub unsafe fn NewParameterizedObject(&self, pConstructor: *mut IUnknown, nTypeArgs: u32, ppTypeArgs: *mut *mut IUnknown, nArgs: u32, ppArgs: *mut *mut IUnknown) -> HRESULT;

    /// Create a new object without constructor with type arguments.
    pub unsafe fn NewParameterizedObjectNoConstructor(&self, pClass: *mut IUnknown, nTypeArgs: u32, ppTypeArgs: *mut *mut IUnknown) -> HRESULT;

    /// Create a new array with type arguments.
    pub unsafe fn NewParameterizedArray(&self, pElementType: *mut IUnknown, rank: u32, dims: *const u32, lowBounds: *const u32) -> HRESULT;

    /// Create a new string with length.
    pub unsafe fn NewStringWithLength(&self, string: *const u16, uiLength: u32) -> HRESULT;

    /// Rude abort the eval.
    pub unsafe fn RudeAbort(&self) -> HRESULT;
}

