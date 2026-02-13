//! ICorDebugModule interface definitions.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// ICorDebugModule - Represents a managed module (DLL or EXE).
#[interface("DBA2D8C1-E5C5-4069-8C13-10A7C6ABF43D")]
pub unsafe trait ICorDebugModule: IUnknown {
    /// Get the process containing this module.
    pub unsafe fn GetProcess(&self, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Get the base address of this module.
    pub unsafe fn GetBaseAddress(&self, pAddress: *mut u64) -> HRESULT;

    /// Get the assembly containing this module.
    pub unsafe fn GetAssembly(&self, ppAssembly: *mut *mut IUnknown) -> HRESULT;

    /// Get the name of this module.
    pub unsafe fn GetName(&self, cchName: u32, pcchName: *mut u32, szName: *mut u16) -> HRESULT;

    /// Enable/disable JIT optimizations.
    pub unsafe fn EnableJITDebugging(&self, bTrackJITInfo: i32, bAllowJitOpts: i32) -> HRESULT;

    /// Enable/disable class load callbacks.
    pub unsafe fn EnableClassLoadCallbacks(&self, bClassLoadCallbacks: i32) -> HRESULT;

    /// Get a function by metadata token.
    pub unsafe fn GetFunctionFromToken(
        &self,
        methodDef: u32,
        ppFunction: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a function by RVA.
    pub unsafe fn GetFunctionFromRVA(&self, rva: u64, ppFunction: *mut *mut IUnknown) -> HRESULT;

    /// Get a class by metadata token.
    pub unsafe fn GetClassFromToken(&self, typeDef: u32, ppClass: *mut *mut IUnknown) -> HRESULT;

    /// Create a breakpoint for this module.
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;

    /// Get the edit and continue snapshot.
    pub unsafe fn GetEditAndContinueSnapshot(
        &self,
        ppEditAndContinueSnapshot: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the metadata interface.
    pub unsafe fn GetMetaDataInterface(
        &self,
        riid: *const windows::core::GUID,
        ppObj: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the token for this module.
    pub unsafe fn GetToken(&self, pToken: *mut u32) -> HRESULT;

    /// Check if this is a dynamic module.
    pub unsafe fn IsDynamic(&self, pDynamic: *mut i32) -> HRESULT;

    /// Get a global variable by metadata token.
    pub unsafe fn GetGlobalVariableValue(
        &self,
        fieldDef: u32,
        ppValue: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the size of this module.
    pub unsafe fn GetSize(&self, pcBytes: *mut u32) -> HRESULT;

    /// Check if this is an in-memory module.
    pub unsafe fn IsInMemory(&self, pInMemory: *mut i32) -> HRESULT;
}

/// ICorDebugModule2 - Extended module interface.
#[interface("7FCC5FB5-49C0-41DE-9938-3B88B5B9ADD7")]
pub unsafe trait ICorDebugModule2: IUnknown {
    /// Set JMC (Just My Code) status.
    pub unsafe fn SetJMCStatus(
        &self,
        bIsJustMyCode: i32,
        cTokens: u32,
        pTokens: *const u32,
    ) -> HRESULT;

    /// Apply changes from Edit and Continue.
    pub unsafe fn ApplyChanges(
        &self,
        cbMetadata: u32,
        pbMetadata: *const u8,
        cbIL: u32,
        pbIL: *const u8,
    ) -> HRESULT;

    /// Set JIT compiler flags.
    pub unsafe fn SetJITCompilerFlags(&self, dwFlags: u32) -> HRESULT;

    /// Get JIT compiler flags.
    pub unsafe fn GetJITCompilerFlags(&self, pdwFlags: *mut u32) -> HRESULT;

    /// Resolve an assembly reference.
    pub unsafe fn ResolveAssembly(
        &self,
        tkAssemblyRef: u32,
        ppAssembly: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// ICorDebugModule3 - Module interface for WinMD.
#[interface("86F012BF-FF15-4372-BD30-B6F11CAAE1DD")]
pub unsafe trait ICorDebugModule3: IUnknown {
    /// Create a reader for symbols.
    pub unsafe fn CreateReaderForInMemorySymbols(
        &self,
        riid: *const windows::core::GUID,
        ppObj: *mut *mut c_void,
    ) -> HRESULT;
}

/// ICorDebugModule4 - Module interface for .NET Native.
#[interface("FF8B8EAF-6E53-4F8E-A8E8-E2355F8A5E5A")]
pub unsafe trait ICorDebugModule4: IUnknown {
    /// Check if this is a mapped module.
    pub unsafe fn IsMappedLayout(&self, pIsMapped: *mut i32) -> HRESULT;
}

/// ICorDebugFunction - Represents a managed function.
#[interface("CC7BCAF6-8A68-11D2-983C-0000F808342D")]
pub unsafe trait ICorDebugFunction: IUnknown {
    /// Get the module containing this function.
    pub unsafe fn GetModule(&self, ppModule: *mut *mut IUnknown) -> HRESULT;

    /// Get the class containing this function.
    pub unsafe fn GetClass(&self, ppClass: *mut *mut IUnknown) -> HRESULT;

    /// Get the metadata token for this function.
    pub unsafe fn GetToken(&self, pMethodDef: *mut u32) -> HRESULT;

    /// Get the IL code for this function.
    pub unsafe fn GetILCode(&self, ppCode: *mut *mut IUnknown) -> HRESULT;

    /// Get the native code for this function.
    pub unsafe fn GetNativeCode(&self, ppCode: *mut *mut IUnknown) -> HRESULT;

    /// Create a breakpoint at the start of this function.
    pub unsafe fn CreateBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;

    /// Get the local variables signature token.
    pub unsafe fn GetLocalVarSigToken(&self, pmdSig: *mut u32) -> HRESULT;

    /// Get the current version number.
    pub unsafe fn GetCurrentVersionNumber(&self, pnCurrentVersion: *mut u32) -> HRESULT;
}

/// ICorDebugFunction2 - Extended function interface.
#[interface("EF0C490B-94C3-4E4D-B629-DDC134C532D8")]
pub unsafe trait ICorDebugFunction2: IUnknown {
    /// Set JMC status for this function.
    pub unsafe fn SetJMCStatus(&self, bIsJustMyCode: i32) -> HRESULT;

    /// Get JMC status for this function.
    pub unsafe fn GetJMCStatus(&self, pbIsJustMyCode: *mut i32) -> HRESULT;

    /// Enumerate native code for this function.
    pub unsafe fn EnumerateNativeCode(&self, ppCodeEnum: *mut *mut IUnknown) -> HRESULT;

    /// Get the version number.
    pub unsafe fn GetVersionNumber(&self, pnVersion: *mut u32) -> HRESULT;
}

/// ICorDebugFunction3 - Function interface for ReJIT.
#[interface("09B70F28-E465-482D-99E0-81A165EB0532")]
pub unsafe trait ICorDebugFunction3: IUnknown {
    /// Get the active ReJIT request IL code.
    pub unsafe fn GetActiveReJitRequestILCode(
        &self,
        ppReJitedILCode: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// ICorDebugFunction4 - Function interface for local variables.
#[interface("72965963-34FD-46E9-9434-B817FE6E7F43")]
pub unsafe trait ICorDebugFunction4: IUnknown {
    /// Create native breakpoint.
    pub unsafe fn CreateNativeBreakpoint(&self, ppBreakpoint: *mut *mut IUnknown) -> HRESULT;
}
