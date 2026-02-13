//! ICorDebugFrame and ICorDebugCode interface definitions.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugMappingResult - Result of IL to native mapping
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugMappingResult {
    MAPPING_PROLOG = 0x01,
    MAPPING_EPILOG = 0x02,
    MAPPING_NO_INFO = 0x04,
    MAPPING_UNMAPPED_ADDRESS = 0x08,
    MAPPING_EXACT = 0x10,
    MAPPING_APPROXIMATE = 0x20,
}

/// ICorDebugFrame - Represents a stack frame.
#[interface("CC7BCAEF-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugFrame: IUnknown {
    /// Get the chain containing this frame.
    pub unsafe fn GetChain(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Get the code for this frame.
    pub unsafe fn GetCode(&self, ppCode: *mut *mut IUnknown) -> HRESULT;

    /// Get the function for this frame.
    pub unsafe fn GetFunction(&self, ppFunction: *mut *mut IUnknown) -> HRESULT;

    /// Get the metadata token for the function.
    pub unsafe fn GetFunctionToken(&self, pToken: *mut u32) -> HRESULT;

    /// Get the stack range for this frame.
    pub unsafe fn GetStackRange(&self, pStart: *mut u64, pEnd: *mut u64) -> HRESULT;

    /// Get the caller frame.
    pub unsafe fn GetCaller(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;

    /// Get the callee frame.
    pub unsafe fn GetCallee(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;

    /// Create a stepper for this frame.
    pub unsafe fn CreateStepper(&self, ppStepper: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugILFrame - Represents an IL stack frame.
#[interface("03E26311-4F76-11D3-88C6-006097945418")]
pub unsafe trait ICorDebugILFrame: IUnknown {
    // ICorDebugFrame methods
    pub unsafe fn GetChain(&self, ppChain: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetCode(&self, ppCode: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetFunction(&self, ppFunction: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetFunctionToken(&self, pToken: *mut u32) -> HRESULT;
    pub unsafe fn GetStackRange(&self, pStart: *mut u64, pEnd: *mut u64) -> HRESULT;
    pub unsafe fn GetCaller(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetCallee(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn CreateStepper(&self, ppStepper: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugILFrame methods
    /// Get the IP (instruction pointer).
    pub unsafe fn GetIP(&self, pnOffset: *mut u32, pMappingResult: *mut u32) -> HRESULT;

    /// Set the IP.
    pub unsafe fn SetIP(&self, nOffset: u32) -> HRESULT;

    /// Enumerate local variables.
    pub unsafe fn EnumerateLocalVariables(&self, ppValueEnum: *mut *mut IUnknown) -> HRESULT;

    /// Get a local variable by index.
    pub unsafe fn GetLocalVariable(&self, dwIndex: u32, ppValue: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate arguments.
    pub unsafe fn EnumerateArguments(&self, ppValueEnum: *mut *mut IUnknown) -> HRESULT;

    /// Get an argument by index.
    pub unsafe fn GetArgument(&self, dwIndex: u32, ppValue: *mut *mut IUnknown) -> HRESULT;

    /// Get the stack depth.
    pub unsafe fn GetStackDepth(&self, pDepth: *mut u32) -> HRESULT;

    /// Get a stack value by index.
    pub unsafe fn GetStackValue(&self, dwIndex: u32, ppValue: *mut *mut IUnknown) -> HRESULT;

    /// Check if SetIP is possible.
    pub unsafe fn CanSetIP(&self, nOffset: u32) -> HRESULT;
}

/// ICorDebugILFrame2 - Extended IL frame interface.
#[interface("5D88A994-6C30-479B-890F-BCEF88B129A5")]
pub unsafe trait ICorDebugILFrame2: IUnknown {
    /// Remap the function.
    pub unsafe fn RemapFunction(&self, newILOffset: u32) -> HRESULT;

    /// Enumerate type parameters.
    pub unsafe fn EnumerateTypeParameters(&self, ppTyParEnum: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugILFrame3 - IL frame interface for interception.
#[interface("9A9E2ED6-04DF-4FE0-BB50-CAB64126619A")]
pub unsafe trait ICorDebugILFrame3: IUnknown {
    /// Get the return value.
    pub unsafe fn GetReturnValueForILOffset(
        &self,
        ilOffset: u32,
        ppReturnValue: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// ICorDebugILFrame4 - IL frame interface for local variables.
#[interface("AD914A30-C6D1-4AC5-9C5E-577F3BAA8A45")]
pub unsafe trait ICorDebugILFrame4: IUnknown {
    /// Enumerate local variables with extra info.
    pub unsafe fn EnumerateLocalVariablesEx(
        &self,
        flags: u32,
        ppValueEnum: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a local variable with extra info.
    pub unsafe fn GetLocalVariableEx(
        &self,
        flags: u32,
        dwIndex: u32,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the code with extra info.
    pub unsafe fn GetCodeEx(&self, flags: u32, ppCode: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugNativeFrame - Represents a native stack frame.
#[interface("03E26314-4F76-11D3-88C6-006097945418")]
pub unsafe trait ICorDebugNativeFrame: IUnknown {
    // ICorDebugFrame methods
    pub unsafe fn GetChain(&self, ppChain: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetCode(&self, ppCode: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetFunction(&self, ppFunction: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetFunctionToken(&self, pToken: *mut u32) -> HRESULT;
    pub unsafe fn GetStackRange(&self, pStart: *mut u64, pEnd: *mut u64) -> HRESULT;
    pub unsafe fn GetCaller(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetCallee(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn CreateStepper(&self, ppStepper: *mut *mut IUnknown) -> HRESULT;

    // ICorDebugNativeFrame methods
    /// Get the IP.
    pub unsafe fn GetIP(&self, pnOffset: *mut u32) -> HRESULT;

    /// Set the IP.
    pub unsafe fn SetIP(&self, nOffset: u32) -> HRESULT;

    /// Get the register set.
    pub unsafe fn GetRegisterSet(&self, ppRegisters: *mut *mut IUnknown) -> HRESULT;

    /// Get a local register value.
    pub unsafe fn GetLocalRegisterValue(
        &self,
        reg: u32,
        cbSigBlob: u32,
        pvSigBlob: *const u8,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a local double register value.
    pub unsafe fn GetLocalDoubleRegisterValue(
        &self,
        highWordReg: u32,
        lowWordReg: u32,
        cbSigBlob: u32,
        pvSigBlob: *const u8,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a local memory value.
    pub unsafe fn GetLocalMemoryValue(
        &self,
        address: u64,
        cbSigBlob: u32,
        pvSigBlob: *const u8,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a local register-memory value.
    pub unsafe fn GetLocalRegisterMemoryValue(
        &self,
        highWordReg: u32,
        lowWordAddress: u64,
        cbSigBlob: u32,
        pvSigBlob: *const u8,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a local memory-register value.
    pub unsafe fn GetLocalMemoryRegisterValue(
        &self,
        highWordAddress: u64,
        lowWordReg: u32,
        cbSigBlob: u32,
        pvSigBlob: *const u8,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Check if SetIP is possible.
    pub unsafe fn CanSetIP(&self, nOffset: u32) -> HRESULT;
}
