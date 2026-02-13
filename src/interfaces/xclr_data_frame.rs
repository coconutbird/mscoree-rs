//! IXCLRDataFrame interface definitions.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::xclr_data_process::CLRDATA_ENUM;

/// IXCLRDataFrame - Represents a stack frame.
#[interface("AB4D17D4-0016-4c1a-9AAF-DA8BE775FBC4")]
pub unsafe trait IXCLRDataFrame: IUnknown {
    /// Get the frame type.
    pub unsafe fn GetFrameType(
        &self,
        simpleType: *mut u32,
        detailedType: *mut u32,
    ) -> HRESULT;

    /// Get the context for this frame.
    pub unsafe fn GetContext(
        &self,
        contextFlags: u32,
        contextBufSize: u32,
        contextSize: *mut u32,
        contextBuf: *mut u8,
    ) -> HRESULT;

    /// Get the method instance for this frame.
    pub unsafe fn GetMethodInstance(
        &self,
        method: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Begin enumeration of local variables.
    pub unsafe fn StartEnumLocalVariables(
        &self,
        flags: u32,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next local variable in the enumeration.
    pub unsafe fn EnumLocalVariable(
        &self,
        handle: *mut CLRDATA_ENUM,
        variable: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of local variables.
    pub unsafe fn EndEnumLocalVariables(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get a local variable by index.
    pub unsafe fn GetLocalVariableByIndex(
        &self,
        index: u32,
        variable: *mut *mut IUnknown,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Begin enumeration of arguments.
    pub unsafe fn StartEnumArguments(
        &self,
        flags: u32,
        handle: *mut CLRDATA_ENUM,
    ) -> HRESULT;

    /// Get the next argument in the enumeration.
    pub unsafe fn EnumArgument(
        &self,
        handle: *mut CLRDATA_ENUM,
        argument: *mut *mut IUnknown,
    ) -> HRESULT;

    /// End enumeration of arguments.
    pub unsafe fn EndEnumArguments(&self, handle: CLRDATA_ENUM) -> HRESULT;

    /// Get an argument by index.
    pub unsafe fn GetArgumentByIndex(
        &self,
        index: u32,
        argument: *mut *mut IUnknown,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the number of arguments.
    pub unsafe fn GetNumArguments(&self, numArgs: *mut u32) -> HRESULT;

    /// Get the number of local variables.
    pub unsafe fn GetNumLocalVariables(&self, numLocals: *mut u32) -> HRESULT;

    /// Get the code name for this frame.
    pub unsafe fn GetCodeName(
        &self,
        flags: u32,
        bufLen: u32,
        nameLen: *mut u32,
        name: *mut u16,
    ) -> HRESULT;

    /// Get the number of type arguments.
    pub unsafe fn GetNumTypeArguments(&self, numTypeArgs: *mut u32) -> HRESULT;

    /// Get a type argument by index.
    pub unsafe fn GetTypeArgumentByIndex(
        &self,
        index: u32,
        typeArg: *mut *mut IUnknown,
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
}

/// IXCLRDataFrame2 - Extended frame interface.
#[interface("1C4D9A4B-702D-4cf6-B290-1DB6F43050D0")]
pub unsafe trait IXCLRDataFrame2: IUnknown {
    /// Get the exact generic arguments for this frame.
    pub unsafe fn GetExactGenericArgsToken(
        &self,
        genericToken: *mut *mut IUnknown,
    ) -> HRESULT;
}

