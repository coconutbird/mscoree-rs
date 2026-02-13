//! ICorDebugCode and ICorDebugChain interface definitions.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// CorDebugChainReason - Reason for a chain
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorDebugChainReason {
    CHAIN_NONE = 0x000,
    CHAIN_CLASS_INIT = 0x001,
    CHAIN_EXCEPTION_FILTER = 0x002,
    CHAIN_SECURITY = 0x004,
    CHAIN_CONTEXT_POLICY = 0x008,
    CHAIN_INTERCEPTION = 0x010,
    CHAIN_PROCESS_START = 0x020,
    CHAIN_THREAD_START = 0x040,
    CHAIN_ENTER_MANAGED = 0x080,
    CHAIN_ENTER_UNMANAGED = 0x100,
    CHAIN_DEBUGGER_EVAL = 0x200,
    CHAIN_CONTEXT_SWITCH = 0x400,
    CHAIN_FUNC_EVAL = 0x800,
}

/// ICorDebugCode - Represents a segment of code.
#[interface("CC7BCAF4-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugCode: IUnknown {
    /// Check if this is IL code.
    pub unsafe fn IsIL(&self, pbIL: *mut i32) -> HRESULT;

    /// Get the function for this code.
    pub unsafe fn GetFunction(&self, ppFunction: *mut *mut IUnknown) -> HRESULT;

    /// Get the address of this code.
    pub unsafe fn GetAddress(&self, pStart: *mut u64) -> HRESULT;

    /// Get the size of this code.
    pub unsafe fn GetSize(&self, pcBytes: *mut u32) -> HRESULT;

    /// Create a breakpoint at an offset.
    pub unsafe fn CreateBreakpoint(&self, offset: u32, ppBreakpoint: *mut *mut IUnknown)
    -> HRESULT;

    /// Get the code bytes.
    pub unsafe fn GetCode(
        &self,
        startOffset: u32,
        endOffset: u32,
        cBufferAlloc: u32,
        buffer: *mut u8,
        pcBufferSize: *mut u32,
    ) -> HRESULT;

    /// Get the version number.
    pub unsafe fn GetVersionNumber(&self, nVersion: *mut u32) -> HRESULT;

    /// Get the IL to native mapping.
    pub unsafe fn GetILToNativeMapping(
        &self,
        cMap: u32,
        pcMap: *mut u32,
        map: *mut c_void,
    ) -> HRESULT;

    /// Get the EnC (Edit and Continue) remapped sequence points.
    pub unsafe fn GetEnCRemapSequencePoints(
        &self,
        cMap: u32,
        pcMap: *mut u32,
        offsets: *mut u32,
    ) -> HRESULT;
}

/// ICorDebugCode2 - Extended code interface.
#[interface("5F696509-452F-4436-A3FE-4D11FE7E2347")]
pub unsafe trait ICorDebugCode2: IUnknown {
    /// Get the code chunks.
    pub unsafe fn GetCodeChunks(
        &self,
        cbufSize: u32,
        pcnumChunks: *mut u32,
        chunks: *mut c_void,
    ) -> HRESULT;

    /// Get the compiler flags.
    pub unsafe fn GetCompilerFlags(&self, pdwFlags: *mut u32) -> HRESULT;
}

/// ICorDebugCode3 - Code interface for return values.
#[interface("D13D3E88-E1F2-4020-AA1D-3D162DCBE966")]
pub unsafe trait ICorDebugCode3: IUnknown {
    /// Get the return value live offset.
    pub unsafe fn GetReturnValueLiveOffset(
        &self,
        ilOffset: u32,
        bufferSize: u32,
        pFetched: *mut u32,
        pOffsets: *mut u32,
    ) -> HRESULT;
}

/// ICorDebugCode4 - Code interface for local variables.
#[interface("18221FA4-20CB-40FA-B19D-9F91C4FA8C14")]
pub unsafe trait ICorDebugCode4: IUnknown {
    /// Enumerate variable homes.
    pub unsafe fn EnumerateVariableHomes(&self, ppEnum: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugILCode - Represents IL code.
#[interface("598D46C2-C877-42A7-89D2-3D0C7F1C1264")]
pub unsafe trait ICorDebugILCode: IUnknown {
    /// Get the EH (exception handling) clauses.
    pub unsafe fn GetEHClauses(
        &self,
        cClauses: u32,
        pcClauses: *mut u32,
        clauses: *mut c_void,
    ) -> HRESULT;
}

/// ICorDebugILCode2 - Extended IL code interface.
#[interface("46586093-D3F5-4DB6-ACDB-955BCE228C15")]
pub unsafe trait ICorDebugILCode2: IUnknown {
    /// Get the local variable signature token.
    pub unsafe fn GetLocalVarSigToken(&self, pmdSig: *mut u32) -> HRESULT;

    /// Get instrumented IL map.
    pub unsafe fn GetInstrumentedILMap(
        &self,
        cMap: u32,
        pcMap: *mut u32,
        map: *mut c_void,
    ) -> HRESULT;
}

/// ICorDebugChain - Represents a chain of stack frames.
#[interface("CC7BCAEE-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugChain: IUnknown {
    /// Get the thread for this chain.
    pub unsafe fn GetThread(&self, ppThread: *mut *mut IUnknown) -> HRESULT;

    /// Get the stack range.
    pub unsafe fn GetStackRange(&self, pStart: *mut u64, pEnd: *mut u64) -> HRESULT;

    /// Get the context.
    pub unsafe fn GetContext(&self, ppContext: *mut *mut IUnknown) -> HRESULT;

    /// Get the caller chain.
    pub unsafe fn GetCaller(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Get the callee chain.
    pub unsafe fn GetCallee(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Get the previous chain.
    pub unsafe fn GetPrevious(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Get the next chain.
    pub unsafe fn GetNext(&self, ppChain: *mut *mut IUnknown) -> HRESULT;

    /// Check if this is a managed chain.
    pub unsafe fn IsManaged(&self, pManaged: *mut i32) -> HRESULT;

    /// Enumerate all frames in this chain.
    pub unsafe fn EnumerateFrames(&self, ppFrames: *mut *mut IUnknown) -> HRESULT;

    /// Get the active frame.
    pub unsafe fn GetActiveFrame(&self, ppFrame: *mut *mut IUnknown) -> HRESULT;

    /// Get the register set.
    pub unsafe fn GetRegisterSet(&self, ppRegisters: *mut *mut IUnknown) -> HRESULT;

    /// Get the reason for this chain.
    pub unsafe fn GetReason(&self, pReason: *mut u32) -> HRESULT;
}

/// ICorDebugRegisterSet - Represents CPU registers.
#[interface("CC7BCB0B-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugRegisterSet: IUnknown {
    /// Get the available registers mask.
    pub unsafe fn GetRegistersAvailable(&self, pAvailable: *mut u64) -> HRESULT;

    /// Get register values.
    pub unsafe fn GetRegisters(&self, mask: u64, regCount: u32, regBuffer: *mut u64) -> HRESULT;

    /// Set register values.
    pub unsafe fn SetRegisters(&self, mask: u64, regCount: u32, regBuffer: *const u64) -> HRESULT;

    /// Get the thread context.
    pub unsafe fn GetThreadContext(&self, contextSize: u32, context: *mut u8) -> HRESULT;

    /// Set the thread context.
    pub unsafe fn SetThreadContext(&self, contextSize: u32, context: *const u8) -> HRESULT;
}

/// ICorDebugRegisterSet2 - Extended register set interface.
#[interface("6DC7BA3F-89BA-4459-9EC1-9D60937B468D")]
pub unsafe trait ICorDebugRegisterSet2: IUnknown {
    /// Get the available registers mask (extended).
    pub unsafe fn GetRegistersAvailable(
        &self,
        numChunks: u32,
        availableRegChunks: *mut u8,
    ) -> HRESULT;

    /// Get register values (extended).
    pub unsafe fn GetRegisters(
        &self,
        maskCount: u32,
        mask: *const u8,
        regCount: u32,
        regBuffer: *mut u64,
    ) -> HRESULT;

    /// Set register values (extended).
    pub unsafe fn SetRegisters(
        &self,
        maskCount: u32,
        mask: *const u8,
        regCount: u32,
        regBuffer: *const u64,
    ) -> HRESULT;
}
