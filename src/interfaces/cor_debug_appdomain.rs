//! ICorDebugAppDomain interface definitions.

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// ICorDebugAppDomain - Represents an application domain.
#[interface("3D6F5F63-7538-11D3-8D5B-00104B35E7EF")]
pub unsafe trait ICorDebugAppDomain: IUnknown {
    // ICorDebugController methods
    pub unsafe fn Stop(&self, dwTimeoutIgnored: u32) -> HRESULT;
    pub unsafe fn Continue(&self, fIsOutOfBand: i32) -> HRESULT;
    pub unsafe fn IsRunning(&self, pbRunning: *mut i32) -> HRESULT;
    pub unsafe fn HasQueuedCallbacks(&self, pThread: *mut IUnknown, pbQueued: *mut i32) -> HRESULT;
    pub unsafe fn EnumerateThreads(&self, ppThreads: *mut *mut IUnknown) -> HRESULT;
    pub unsafe fn SetAllThreadsDebugState(
        &self,
        state: u32,
        pExceptThisThread: *mut IUnknown,
    ) -> HRESULT;
    pub unsafe fn Detach(&self) -> HRESULT;
    pub unsafe fn Terminate(&self, exitCode: u32) -> HRESULT;
    pub unsafe fn CanCommitChanges(
        &self,
        cSnapshots: u32,
        pSnapshots: *mut *mut IUnknown,
        pError: *mut *mut IUnknown,
    ) -> HRESULT;
    pub unsafe fn CommitChanges(
        &self,
        cSnapshots: u32,
        pSnapshots: *mut *mut IUnknown,
        pError: *mut *mut IUnknown,
    ) -> HRESULT;

    // ICorDebugAppDomain methods
    /// Get the process containing this app domain.
    pub unsafe fn GetProcess(&self, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate all assemblies in this app domain.
    pub unsafe fn EnumerateAssemblies(&self, ppAssemblies: *mut *mut IUnknown) -> HRESULT;

    /// Get the module containing the metadata token.
    pub unsafe fn GetModuleFromMetaDataInterface(
        &self,
        pIMetaData: *mut IUnknown,
        ppModule: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Enumerate all breakpoints in this app domain.
    pub unsafe fn EnumerateBreakpoints(&self, ppBreakpoints: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate all steppers in this app domain.
    pub unsafe fn EnumerateSteppers(&self, ppSteppers: *mut *mut IUnknown) -> HRESULT;

    /// Check if the debugger is attached.
    pub unsafe fn IsAttached(&self, pbAttached: *mut i32) -> HRESULT;

    /// Get the name of this app domain.
    pub unsafe fn GetName(&self, cchName: u32, pcchName: *mut u32, szName: *mut u16) -> HRESULT;

    /// Get the object representing this app domain.
    pub unsafe fn GetObject(&self, ppObject: *mut *mut IUnknown) -> HRESULT;

    /// Attach to this app domain.
    pub unsafe fn Attach(&self) -> HRESULT;

    /// Get the ID of this app domain.
    pub unsafe fn GetID(&self, pId: *mut u32) -> HRESULT;
}

/// ICorDebugAppDomain2 - Extended app domain interface.
#[interface("096E81D5-ECDA-4202-83F5-C65980A9EF75")]
pub unsafe trait ICorDebugAppDomain2: IUnknown {
    /// Get the array or pointer type.
    pub unsafe fn GetArrayOrPointerType(
        &self,
        elementType: u32,
        nRank: u32,
        pTypeArg: *mut IUnknown,
        ppType: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get a function pointer type.
    pub unsafe fn GetFunctionPointerType(
        &self,
        nTypeArgs: u32,
        ppTypeArgs: *mut *mut IUnknown,
        ppType: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// ICorDebugAppDomain3 - App domain interface for WinRT.
#[interface("8CB96A16-B588-42E2-B71C-DD849FC2ECCC")]
pub unsafe trait ICorDebugAppDomain3: IUnknown {
    /// Get cached WinRT types.
    pub unsafe fn GetCachedWinRTTypesForIIDs(
        &self,
        cReqTypes: u32,
        iidsToResolve: *const windows::core::GUID,
        ppTypesEnum: *mut *mut IUnknown,
    ) -> HRESULT;

    /// Get cached WinRT types.
    pub unsafe fn GetCachedWinRTTypes(&self, ppGuidToTypeEnum: *mut *mut IUnknown) -> HRESULT;
}

/// ICorDebugAppDomain4 - App domain interface for COM callable wrappers.
#[interface("FB99CC40-83BE-4724-AB3B-768E796EBAC2")]
pub unsafe trait ICorDebugAppDomain4: IUnknown {
    /// Get the object for a COM callable wrapper.
    pub unsafe fn GetObjectForCCW(
        &self,
        ccwPointer: u64,
        ppManagedObject: *mut *mut IUnknown,
    ) -> HRESULT;
}

/// ICorDebugAssembly - Represents an assembly.
#[interface("DF59507C-D47A-459E-BCE2-6427EAC8FD06")]
pub unsafe trait ICorDebugAssembly: IUnknown {
    /// Get the process containing this assembly.
    pub unsafe fn GetProcess(&self, ppProcess: *mut *mut IUnknown) -> HRESULT;

    /// Get the app domain containing this assembly.
    pub unsafe fn GetAppDomain(&self, ppAppDomain: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate all modules in this assembly.
    pub unsafe fn EnumerateModules(&self, ppModules: *mut *mut IUnknown) -> HRESULT;

    /// Get the code base of this assembly.
    pub unsafe fn GetCodeBase(&self, cchName: u32, pcchName: *mut u32, szName: *mut u16)
    -> HRESULT;

    /// Get the name of this assembly.
    pub unsafe fn GetName(&self, cchName: u32, pcchName: *mut u32, szName: *mut u16) -> HRESULT;
}

/// ICorDebugAssembly2 - Extended assembly interface.
#[interface("426D1F9E-6DD4-44C8-AEC7-26CDBAF4E398")]
pub unsafe trait ICorDebugAssembly2: IUnknown {
    /// Check if this is a fully trusted assembly.
    pub unsafe fn IsFullyTrusted(&self, pbFullyTrusted: *mut i32) -> HRESULT;
}

/// ICorDebugAssembly3 - Assembly interface for container assemblies.
#[interface("76361AB2-8C86-4FE9-96F2-F73D8843570A")]
pub unsafe trait ICorDebugAssembly3: IUnknown {
    /// Get the container assembly.
    pub unsafe fn GetContainerAssembly(&self, ppAssembly: *mut *mut IUnknown) -> HRESULT;

    /// Enumerate contained assemblies.
    pub unsafe fn EnumerateContainedAssemblies(&self, ppAssemblies: *mut *mut IUnknown) -> HRESULT;
}
