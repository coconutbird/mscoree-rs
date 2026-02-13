//! IXCLRDataTask interface definition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IXCLRDataTask - Represents a managed task (thread/fiber).
#[interface("A5B0BEEA-EC62-4618-8012-A24FFC23934C")]
pub unsafe trait IXCLRDataTask: IUnknown {
    /// Get the owning process.
    pub unsafe fn GetProcess(
        &self,
        process: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the current app domain for this task.
    pub unsafe fn GetCurrentAppDomain(
        &self,
        appDomain: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the unique identifier for this task.
    pub unsafe fn GetUniqueID(&self, id: *mut u64) -> HRESULT;

    /// Get the flags for this task.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Determine whether the given interface represents the same target state.
    pub unsafe fn IsSameObject(&self, task: *mut IXCLRDataTask) -> HRESULT;

    /// Get the managed object representing this task.
    pub unsafe fn GetManagedObject(&self, value: *mut *mut IUnknown) -> HRESULT;

    /// Get the desired execution state.
    pub unsafe fn GetDesiredExecutionState(&self, state: *mut u32) -> HRESULT;

    /// Set the desired execution state.
    pub unsafe fn SetDesiredExecutionState(&self, state: u32) -> HRESULT;

    /// Create a stack walker for this task.
    pub unsafe fn CreateStackWalk(
        &self,
        flags: u32,
        stackWalk: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the OS thread ID.
    pub unsafe fn GetOSThreadID(&self, id: *mut u32) -> HRESULT;

    /// Get the context for this task.
    pub unsafe fn GetContext(
        &self,
        contextFlags: u32,
        contextBufSize: u32,
        contextSize: *mut u32,
        contextBuf: *mut u8,
    ) -> HRESULT;

    /// Set the context for this task.
    pub unsafe fn SetContext(
        &self,
        contextSize: u32,
        context: *const u8,
    ) -> HRESULT;

    /// Get the current exception state.
    pub unsafe fn GetCurrentExceptionState(
        &self,
        exception: *mut *mut IUnknown,
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

    /// Get the name of this task.
    pub unsafe fn GetName(
        &self,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the last exception state.
    pub unsafe fn GetLastExceptionState(
        &self,
        exception: *mut *mut IUnknown,
    ) -> HRESULT;
}

