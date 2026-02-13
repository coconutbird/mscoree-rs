//! IXCLRDataStackWalk interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IXCLRDataStackWalk - Interface for walking the managed stack.
#[interface("E59D8D22-ADA7-49a2-89B5-A415AFCFC95F")]
pub unsafe trait IXCLRDataStackWalk: IUnknown {
    /// Get the context for the current frame.
    pub unsafe fn GetContext(
        &self,
        contextFlags: u32,
        contextBufSize: u32,
        contextSize: *mut u32,
        contextBuf: *mut u8,
    ) -> HRESULT;

    /// Set the context for the walker.
    pub unsafe fn SetContext(
        &self,
        contextSize: u32,
        context: *const u8,
    ) -> HRESULT;

    /// Move to the next frame.
    pub unsafe fn Next(&self) -> HRESULT;

    /// Get the amount of stack space skipped.
    pub unsafe fn GetStackSizeSkipped(&self, stackSizeSkipped: *mut u64) -> HRESULT;

    /// Get the frame type.
    pub unsafe fn GetFrameType(
        &self,
        simpleType: *mut u32,
        detailedType: *mut u32,
    ) -> HRESULT;

    /// Get the current frame.
    pub unsafe fn GetFrame(
        &self,
        frame: *mut *mut IUnknown,
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

    /// Set the context with additional control.
    pub unsafe fn SetContext2(
        &self,
        flags: u32,
        contextSize: u32,
        context: *const u8,
    ) -> HRESULT;
}

