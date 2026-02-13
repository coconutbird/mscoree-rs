//! ICorDebugValue interface definitions.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorElementType - Element types for values
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorElementType {
    ELEMENT_TYPE_END = 0x00,
    ELEMENT_TYPE_VOID = 0x01,
    ELEMENT_TYPE_BOOLEAN = 0x02,
    ELEMENT_TYPE_CHAR = 0x03,
    ELEMENT_TYPE_I1 = 0x04,
    ELEMENT_TYPE_U1 = 0x05,
    ELEMENT_TYPE_I2 = 0x06,
    ELEMENT_TYPE_U2 = 0x07,
    ELEMENT_TYPE_I4 = 0x08,
    ELEMENT_TYPE_U4 = 0x09,
    ELEMENT_TYPE_I8 = 0x0a,
    ELEMENT_TYPE_U8 = 0x0b,
    ELEMENT_TYPE_R4 = 0x0c,
    ELEMENT_TYPE_R8 = 0x0d,
    ELEMENT_TYPE_STRING = 0x0e,
    ELEMENT_TYPE_PTR = 0x0f,
    ELEMENT_TYPE_BYREF = 0x10,
    ELEMENT_TYPE_VALUETYPE = 0x11,
    ELEMENT_TYPE_CLASS = 0x12,
    ELEMENT_TYPE_VAR = 0x13,
    ELEMENT_TYPE_ARRAY = 0x14,
    ELEMENT_TYPE_GENERICINST = 0x15,
    ELEMENT_TYPE_TYPEDBYREF = 0x16,
    ELEMENT_TYPE_I = 0x18,
    ELEMENT_TYPE_U = 0x19,
    ELEMENT_TYPE_FNPTR = 0x1b,
    ELEMENT_TYPE_OBJECT = 0x1c,
    ELEMENT_TYPE_SZARRAY = 0x1d,
    ELEMENT_TYPE_MVAR = 0x1e,
}

/// ICorDebugValue - Represents a value in the debuggee.
#[interface("CC7BCAF7-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugValue: IUnknown {
    /// Get the type of this value.
    pub unsafe fn GetType(&self, pType: *mut u32) -> HRESULT;

    /// Get the size of this value.
    pub unsafe fn GetSize(&self, pSize: *mut u32) -> HRESULT;

    /// Get the address of this value.
    pub unsafe fn GetAddress(&self, pAddress: *mut u64) -> HRESULT;

    /// Create a breakpoint on this value.
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugValue2 - Extended value interface.
#[interface("5E0B54E7-D88A-4626-9420-A691E0A78B49")]
pub unsafe trait ICorDebugValue2: IUnknown {
    /// Get the exact type of this value.
    pub unsafe fn GetExactType(&self, ppType: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugValue3 - Value interface for large values.
#[interface("565005FC-0F8A-4F3E-9EDB-83102B156595")]
pub unsafe trait ICorDebugValue3: IUnknown {
    /// Get the size as 64-bit.
    pub unsafe fn GetSize64(&self, pSize: *mut u64) -> HRESULT;
}

/// ICorDebugGenericValue - Represents a primitive value.
#[interface("CC7BCAF8-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugGenericValue: IUnknown {
    // ICorDebugValue methods
    pub unsafe fn GetType(&self, pType: *mut u32) -> HRESULT;
    pub unsafe fn GetSize(&self, pSize: *mut u32) -> HRESULT;
    pub unsafe fn GetAddress(&self, pAddress: *mut u64) -> HRESULT;
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugGenericValue methods
    /// Get the value.
    pub unsafe fn GetValue(&self, pTo: *mut u8) -> HRESULT;

    /// Set the value.
    pub unsafe fn SetValue(&self, pFrom: *const u8) -> HRESULT;
}

/// ICorDebugReferenceValue - Represents a reference value.
#[interface("CC7BCAF9-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugReferenceValue: IUnknown {
    // ICorDebugValue methods
    pub unsafe fn GetType(&self, pType: *mut u32) -> HRESULT;
    pub unsafe fn GetSize(&self, pSize: *mut u32) -> HRESULT;
    pub unsafe fn GetAddress(&self, pAddress: *mut u64) -> HRESULT;
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugReferenceValue methods
    /// Check if this is null.
    pub unsafe fn IsNull(&self, pbNull: *mut i32) -> HRESULT;

    /// Get the value of the reference.
    pub unsafe fn GetValue(&self, pValue: *mut u64) -> HRESULT;

    /// Set the value of the reference.
    pub unsafe fn SetValue(&self, value: u64) -> HRESULT;

    /// Dereference this reference.
    pub unsafe fn Dereference(&self, ppValue: *mut *mut IUnknown) -> HRESULT;

    /// Dereference strongly typed.
    pub unsafe fn DereferenceStrong(&self, ppValue: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugHeapValue - Represents a heap value.
#[interface("CC7BCAFA-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugHeapValue: IUnknown {
    // ICorDebugValue methods
    pub unsafe fn GetType(&self, pType: *mut u32) -> HRESULT;
    pub unsafe fn GetSize(&self, pSize: *mut u32) -> HRESULT;
    pub unsafe fn GetAddress(&self, pAddress: *mut u64) -> HRESULT;
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugHeapValue methods
    /// Check if this value is valid.
    pub unsafe fn IsValid(&self, pbValid: *mut i32) -> HRESULT;

    /// Create a relocate breakpoint.
    pub unsafe fn CreateRelocBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugHeapValue2 - Extended heap value interface.
#[interface("E3AC4D6C-9CB7-43E6-96CC-B21540E5083C")]
pub unsafe trait ICorDebugHeapValue2: IUnknown {
    /// Create a handle for this value.
    pub unsafe fn CreateHandle(&self, r#type: u32, ppHandle: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugHeapValue3 - Heap value interface for monitor info.
#[interface("A69ACAD8-2374-46E9-9FF8-B1F14120D296")]
pub unsafe trait ICorDebugHeapValue3: IUnknown {
    /// Get the thread owning the monitor lock.
    pub unsafe fn GetThreadOwningMonitorLock(&self, ppThread: *mut *mut IUnknown, pAcquisitionCount: *mut u32) -> HRESULT;

    /// Get threads waiting on the monitor lock.
    pub unsafe fn GetMonitorEventWaitList(&self, ppThreadEnum: *mut *mut IUnknown) -> HRESULT;
}

