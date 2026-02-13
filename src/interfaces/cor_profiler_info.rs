//! ICorProfilerInfo interface definitions for profiler queries.
//!
//! These interfaces are used by profilers to query the CLR runtime.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// ICorProfilerInfo - Query interface for profilers.
#[interface("28B5557D-3F3F-48B4-90B2-5F9EEA2F6C48")]
pub unsafe trait ICorProfilerInfo: IUnknown {
    /// Get the class from an object.
    pub unsafe fn GetClassFromObject(&self, objectId: usize, pClassId: *mut usize) -> HRESULT;

    /// Get the class from a token.
    pub unsafe fn GetClassFromToken(
        &self,
        moduleId: usize,
        typeDef: u32,
        pClassId: *mut usize,
    ) -> HRESULT;

    /// Get the code info.
    pub unsafe fn GetCodeInfo(
        &self,
        functionId: usize,
        pStart: *mut *const u8,
        pcSize: *mut u32,
    ) -> HRESULT;

    /// Get the event mask.
    pub unsafe fn GetEventMask(&self, pdwEvents: *mut u32) -> HRESULT;

    /// Get the function from IP.
    pub unsafe fn GetFunctionFromIP(&self, ip: *const u8, pFunctionId: *mut usize) -> HRESULT;

    /// Get the function from token.
    pub unsafe fn GetFunctionFromToken(
        &self,
        moduleId: usize,
        token: u32,
        pFunctionId: *mut usize,
    ) -> HRESULT;

    /// Get the handle from thread.
    pub unsafe fn GetHandleFromThread(
        &self,
        threadId: usize,
        phThread: *mut *mut c_void,
    ) -> HRESULT;

    /// Get the object size.
    pub unsafe fn GetObjectSize(&self, objectId: usize, pcSize: *mut u32) -> HRESULT;

    /// Check if the function is JIT compiled.
    pub unsafe fn IsArrayClass(
        &self,
        classId: usize,
        pBaseElemType: *mut u32,
        pBaseClassId: *mut usize,
        pcRank: *mut u32,
    ) -> HRESULT;

    /// Get the thread info.
    pub unsafe fn GetThreadInfo(&self, threadId: usize, pdwWin32ThreadId: *mut u32) -> HRESULT;

    /// Get the current thread ID.
    pub unsafe fn GetCurrentThreadID(&self, pThreadId: *mut usize) -> HRESULT;

    /// Get the class ID info.
    pub unsafe fn GetClassIDInfo(
        &self,
        classId: usize,
        pModuleId: *mut usize,
        pTypeDefToken: *mut u32,
    ) -> HRESULT;

    /// Get the function info.
    pub unsafe fn GetFunctionInfo(
        &self,
        functionId: usize,
        pClassId: *mut usize,
        pModuleId: *mut usize,
        pToken: *mut u32,
    ) -> HRESULT;

    /// Set the event mask.
    pub unsafe fn SetEventMask(&self, dwEvents: u32) -> HRESULT;

    /// Set the enter/leave function hooks.
    pub unsafe fn SetEnterLeaveFunctionHooks(
        &self,
        pFuncEnter: *const c_void,
        pFuncLeave: *const c_void,
        pFuncTailcall: *const c_void,
    ) -> HRESULT;

    /// Set the function ID mapper.
    pub unsafe fn SetFunctionIDMapper(&self, pFunc: *const c_void) -> HRESULT;

    /// Get token and metadata.
    pub unsafe fn GetTokenAndMetaDataFromFunction(
        &self,
        functionId: usize,
        riid: *const GUID,
        ppImport: *mut *mut IUnknown,
        pToken: *mut u32,
    ) -> HRESULT;

    /// Get the module info.
    pub unsafe fn GetModuleInfo(
        &self,
        moduleId: usize,
        ppBaseLoadAddress: *mut *const u8,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pAssemblyId: *mut usize,
    ) -> HRESULT;

    /// Get the module metadata.
    pub unsafe fn GetModuleMetaData(
        &self,
        moduleId: usize,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppOut: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get the IL function body.
    pub unsafe fn GetILFunctionBody(
        &self,
        moduleId: usize,
        methodId: u32,
        ppMethodHeader: *mut *const u8,
        pcbMethodSize: *mut u32,
    ) -> HRESULT;

    /// Get IL function body allocator.
    pub unsafe fn GetILFunctionBodyAllocator(
        &self,
        moduleId: usize,
        ppMalloc: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Set the IL function body.
    pub unsafe fn SetILFunctionBody(
        &self,
        moduleId: usize,
        methodid: u32,
        pbNewILMethodHeader: *const u8,
    ) -> HRESULT;

    /// Get the app domain info.
    pub unsafe fn GetAppDomainInfo(
        &self,
        appDomainId: usize,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pProcessId: *mut usize,
    ) -> HRESULT;

    /// Get the assembly info.
    pub unsafe fn GetAssemblyInfo(
        &self,
        assemblyId: usize,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pAppDomainId: *mut usize,
        pModuleId: *mut usize,
    ) -> HRESULT;

    /// Set the function rejit flag.
    pub unsafe fn SetFunctionReJIT(&self, functionId: usize) -> HRESULT;

    /// Force GC.
    pub unsafe fn ForceGC(&self) -> HRESULT;

    /// Set IL instrumented code map.
    pub unsafe fn SetILInstrumentedCodeMap(
        &self,
        functionId: usize,
        fStartJit: i32,
        cILMapEntries: u32,
        rgILMapEntries: *const c_void,
    ) -> HRESULT;

    /// Get inproc inspection interface.
    pub unsafe fn GetInprocInspectionInterface(&self, ppicd: *mut *mut IUnknown) -> HRESULT;

    /// Get inproc inspection this thread.
    pub unsafe fn GetInprocInspectionIThisThread(&self, ppicd: *mut *mut IUnknown) -> HRESULT;

    /// Get thread context.
    pub unsafe fn GetThreadContext(&self, threadId: usize, pContextId: *mut usize) -> HRESULT;

    /// Begin inproc debugging.
    pub unsafe fn BeginInprocDebugging(
        &self,
        fThisThreadOnly: i32,
        pdwProfilerContext: *mut u32,
    ) -> HRESULT;

    /// End inproc debugging.
    pub unsafe fn EndInprocDebugging(&self, dwProfilerContext: u32) -> HRESULT;

    /// Get IL to native mapping.
    pub unsafe fn GetILToNativeMapping(
        &self,
        functionId: usize,
        cMap: u32,
        pcMap: *mut u32,
        map: *mut c_void,
    ) -> HRESULT;
}

/// ICorProfilerInfo2 - Extended query interface for profilers.
#[interface("CC0935CD-A518-487D-B0BB-A93214E65478")]
pub unsafe trait ICorProfilerInfo2: IUnknown {
    // ICorProfilerInfo methods (inherited, not repeated here for brevity)
    pub unsafe fn GetClassFromObject(&self, objectId: usize, pClassId: *mut usize) -> HRESULT;
    pub unsafe fn GetClassFromToken(
        &self,
        moduleId: usize,
        typeDef: u32,
        pClassId: *mut usize,
    ) -> HRESULT;
    pub unsafe fn GetCodeInfo(
        &self,
        functionId: usize,
        pStart: *mut *const u8,
        pcSize: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetEventMask(&self, pdwEvents: *mut u32) -> HRESULT;
    pub unsafe fn GetFunctionFromIP(&self, ip: *const u8, pFunctionId: *mut usize) -> HRESULT;
    pub unsafe fn GetFunctionFromToken(
        &self,
        moduleId: usize,
        token: u32,
        pFunctionId: *mut usize,
    ) -> HRESULT;
    pub unsafe fn GetHandleFromThread(
        &self,
        threadId: usize,
        phThread: *mut *mut c_void,
    ) -> HRESULT;
    pub unsafe fn GetObjectSize(&self, objectId: usize, pcSize: *mut u32) -> HRESULT;
    pub unsafe fn IsArrayClass(
        &self,
        classId: usize,
        pBaseElemType: *mut u32,
        pBaseClassId: *mut usize,
        pcRank: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetThreadInfo(&self, threadId: usize, pdwWin32ThreadId: *mut u32) -> HRESULT;
    pub unsafe fn GetCurrentThreadID(&self, pThreadId: *mut usize) -> HRESULT;
    pub unsafe fn GetClassIDInfo(
        &self,
        classId: usize,
        pModuleId: *mut usize,
        pTypeDefToken: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFunctionInfo(
        &self,
        functionId: usize,
        pClassId: *mut usize,
        pModuleId: *mut usize,
        pToken: *mut u32,
    ) -> HRESULT;
    pub unsafe fn SetEventMask(&self, dwEvents: u32) -> HRESULT;
    pub unsafe fn SetEnterLeaveFunctionHooks(
        &self,
        pFuncEnter: *const c_void,
        pFuncLeave: *const c_void,
        pFuncTailcall: *const c_void,
    ) -> HRESULT;
    pub unsafe fn SetFunctionIDMapper(&self, pFunc: *const c_void) -> HRESULT;
    pub unsafe fn GetTokenAndMetaDataFromFunction(
        &self,
        functionId: usize,
        riid: *const GUID,
        ppImport: *mut *mut IUnknown,
        pToken: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetModuleInfo(
        &self,
        moduleId: usize,
        ppBaseLoadAddress: *mut *const u8,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pAssemblyId: *mut usize,
    ) -> HRESULT;
    pub unsafe fn GetModuleMetaData(
        &self,
        moduleId: usize,
        dwOpenFlags: u32,
        riid: *const GUID,
        ppOut: *mut *mut IUnknown,
    ) -> HRESULT;
    pub unsafe fn GetILFunctionBody(
        &self,
        moduleId: usize,
        methodId: u32,
        ppMethodHeader: *mut *const u8,
        pcbMethodSize: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetILFunctionBodyAllocator(
        &self,
        moduleId: usize,
        ppMalloc: *mut *mut IUnknown,
    ) -> HRESULT;
    pub unsafe fn SetILFunctionBody(
        &self,
        moduleId: usize,
        methodid: u32,
        pbNewILMethodHeader: *const u8,
    ) -> HRESULT;
    pub unsafe fn GetAppDomainInfo(
        &self,
        appDomainId: usize,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pProcessId: *mut usize,
    ) -> HRESULT;
    pub unsafe fn GetAssemblyInfo(
        &self,
        assemblyId: usize,
        cchName: u32,
        pcchName: *mut u32,
        szName: *mut u16,
        pAppDomainId: *mut usize,
        pModuleId: *mut usize,
    ) -> HRESULT;
    pub unsafe fn SetFunctionReJIT(&self, functionId: usize) -> HRESULT;
    pub unsafe fn ForceGC(&self) -> HRESULT;
    pub unsafe fn SetILInstrumentedCodeMap(
        &self,
        functionId: usize,
        fStartJit: i32,
        cILMapEntries: u32,
        rgILMapEntries: *const c_void,
    ) -> HRESULT;
    pub unsafe fn GetInprocInspectionInterface(&self, ppicd: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetInprocInspectionIThisThread(&self, ppicd: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn GetThreadContext(&self, threadId: usize, pContextId: *mut usize) -> HRESULT;
    pub unsafe fn BeginInprocDebugging(
        &self,
        fThisThreadOnly: i32,
        pdwProfilerContext: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EndInprocDebugging(&self, dwProfilerContext: u32) -> HRESULT;
    pub unsafe fn GetILToNativeMapping(
        &self,
        functionId: usize,
        cMap: u32,
        pcMap: *mut u32,
        map: *mut c_void,
    ) -> HRESULT;

    // ICorProfilerInfo2 specific methods
    /// Do stack snapshot.
    pub unsafe fn DoStackSnapshot(
        &self,
        thread: usize,
        callback: *const c_void,
        infoFlags: u32,
        clientData: *mut c_void,
        context: *const u8,
        contextSize: u32,
    ) -> HRESULT;

    /// Set enter/leave hooks with info.
    pub unsafe fn SetEnterLeaveFunctionHooks2(
        &self,
        pFuncEnter: *const c_void,
        pFuncLeave: *const c_void,
        pFuncTailcall: *const c_void,
    ) -> HRESULT;

    /// Get function info with parent class.
    pub unsafe fn GetFunctionInfo2(
        &self,
        funcId: usize,
        frameInfo: usize,
        pClassId: *mut usize,
        pModuleId: *mut usize,
        pToken: *mut u32,
        cTypeArgs: u32,
        pcTypeArgs: *mut u32,
        typeArgs: *mut usize,
    ) -> HRESULT;

    /// Get string layout.
    pub unsafe fn GetStringLayout(
        &self,
        pBufferLengthOffset: *mut u32,
        pStringLengthOffset: *mut u32,
        pBufferOffset: *mut u32,
    ) -> HRESULT;

    /// Get class layout.
    pub unsafe fn GetClassLayout(
        &self,
        classID: usize,
        rFieldOffset: *mut c_void,
        cFieldOffset: u32,
        pcFieldOffset: *mut u32,
        pulClassSize: *mut u32,
    ) -> HRESULT;

    /// Get class ID info.
    pub unsafe fn GetClassIDInfo2(
        &self,
        classId: usize,
        pModuleId: *mut usize,
        pTypeDefToken: *mut u32,
        pParentClassId: *mut usize,
        cNumTypeArgs: u32,
        pcNumTypeArgs: *mut u32,
        typeArgs: *mut usize,
    ) -> HRESULT;

    /// Get code info 2.
    pub unsafe fn GetCodeInfo2(
        &self,
        functionID: usize,
        cCodeInfos: u32,
        pcCodeInfos: *mut u32,
        codeInfos: *mut c_void,
    ) -> HRESULT;

    /// Get class from token and type args.
    pub unsafe fn GetClassFromTokenAndTypeArgs(
        &self,
        moduleID: usize,
        typeDef: u32,
        cTypeArgs: u32,
        typeArgs: *const usize,
        pClassID: *mut usize,
    ) -> HRESULT;

    /// Get function from token and type args.
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(
        &self,
        moduleID: usize,
        funcDef: u32,
        classId: usize,
        cTypeArgs: u32,
        typeArgs: *const usize,
        pFunctionID: *mut usize,
    ) -> HRESULT;

    /// Enumerate frozen objects.
    pub unsafe fn EnumModuleFrozenObjects(
        &self,
        moduleID: usize,
        ppEnum: *mut *mut c_void,
    ) -> HRESULT;

    /// Get array object info.
    pub unsafe fn GetArrayObjectInfo(
        &self,
        objectId: usize,
        cDimensions: u32,
        pDimensionSizes: *mut u32,
        pDimensionLowerBounds: *mut i32,
        ppData: *mut *mut u8,
    ) -> HRESULT;

    /// Get box class layout.
    pub unsafe fn GetBoxClassLayout(&self, classId: usize, pBufferOffset: *mut u32) -> HRESULT;

    /// Get thread app domain.
    pub unsafe fn GetThreadAppDomain(&self, threadId: usize, pAppDomainId: *mut usize) -> HRESULT;

    /// Get RVA static address.
    pub unsafe fn GetRVAStaticAddress(
        &self,
        classId: usize,
        fieldToken: u32,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT;

    /// Get app domain static address.
    pub unsafe fn GetAppDomainStaticAddress(
        &self,
        classId: usize,
        fieldToken: u32,
        appDomainId: usize,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT;

    /// Get thread static address.
    pub unsafe fn GetThreadStaticAddress(
        &self,
        classId: usize,
        fieldToken: u32,
        threadId: usize,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT;

    /// Get context static address.
    pub unsafe fn GetContextStaticAddress(
        &self,
        classId: usize,
        fieldToken: u32,
        contextId: usize,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT;

    /// Get static field info.
    pub unsafe fn GetStaticFieldInfo(
        &self,
        classId: usize,
        fieldToken: u32,
        pFieldInfo: *mut u32,
    ) -> HRESULT;

    /// Get generation bounds.
    pub unsafe fn GetGenerationBounds(
        &self,
        cObjectRanges: u32,
        pcObjectRanges: *mut u32,
        ranges: *mut c_void,
    ) -> HRESULT;

    /// Get object generation.
    pub unsafe fn GetObjectGeneration(&self, objectId: usize, range: *mut c_void) -> HRESULT;

    /// Get notified exception clause info.
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut c_void) -> HRESULT;
}
