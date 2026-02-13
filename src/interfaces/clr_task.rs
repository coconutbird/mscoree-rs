//! ICLRTask and related task/threading hosting interfaces.
//!
//! These interfaces allow custom thread management by hosts.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// Wait operation enumeration.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WAIT_OPTION {
    WAIT_MSGPUMP = 0x1,
    WAIT_ALERTABLE = 0x2,
    WAIT_NOTINDEADLOCK = 0x4,
}

/// ICLRTask - Represents a CLR task (managed thread).
#[interface("28E66A4A-9906-4225-B231-9187C3EB8611")]
pub unsafe trait ICLRTask: IUnknown {
    /// Switch into this task.
    pub unsafe fn SwitchIn(&self, threadHandle: *mut c_void) -> HRESULT;

    /// Switch out of this task.
    pub unsafe fn SwitchOut(&self) -> HRESULT;

    /// Get the task's managed thread info.
    pub unsafe fn GetMemStats(&self, pMemUsage: *mut c_void) -> HRESULT;

    /// Reset this task.
    pub unsafe fn Reset(&self, fFull: i32) -> HRESULT;

    /// Exit this task.
    pub unsafe fn ExitTask(&self) -> HRESULT;

    /// Abort this task.
    pub unsafe fn Abort(&self) -> HRESULT;

    /// Rude abort this task.
    pub unsafe fn RudeAbort(&self) -> HRESULT;

    /// Needs priority scheduling.
    pub unsafe fn NeedsPriorityScheduling(&self, pbNeedsPriorityScheduling: *mut i32) -> HRESULT;

    /// Yield the task.
    pub unsafe fn YieldTask(&self) -> HRESULT;

    /// Locate lock owner.
    pub unsafe fn LocksHeld(
        &self,
        pLockCount: *mut usize,
        pOwningTaskCount: *mut usize,
    ) -> HRESULT;

    /// Set task identifier.
    pub unsafe fn SetTaskIdentifier(&self, dwId: u64) -> HRESULT;
}

/// ICLRTask2 - Extended task interface.
#[interface("28E66A4A-9906-4225-B231-9187C3EB8612")]
pub unsafe trait ICLRTask2: IUnknown {
    // ICLRTask methods
    pub unsafe fn SwitchIn(&self, threadHandle: *mut c_void) -> HRESULT;
    pub unsafe fn SwitchOut(&self) -> HRESULT;
    pub unsafe fn GetMemStats(&self, pMemUsage: *mut c_void) -> HRESULT;
    pub unsafe fn Reset(&self, fFull: i32) -> HRESULT;
    pub unsafe fn ExitTask(&self) -> HRESULT;
    pub unsafe fn Abort(&self) -> HRESULT;
    pub unsafe fn RudeAbort(&self) -> HRESULT;
    pub unsafe fn NeedsPriorityScheduling(&self, pbNeedsPriorityScheduling: *mut i32) -> HRESULT;
    pub unsafe fn YieldTask(&self) -> HRESULT;
    pub unsafe fn LocksHeld(&self, pLockCount: *mut usize, pOwningTaskCount: *mut usize) -> HRESULT;
    pub unsafe fn SetTaskIdentifier(&self, dwId: u64) -> HRESULT;

    // ICLRTask2 specific
    /// Begin prevent async abort.
    pub unsafe fn BeginPreventAsyncAbort(&self) -> HRESULT;

    /// End prevent async abort.
    pub unsafe fn EndPreventAsyncAbort(&self) -> HRESULT;
}

/// IHostTask - Host implementation of a task.
#[interface("C2275828-C4B1-4B55-82C9-92135F74DF1A")]
pub unsafe trait IHostTask: IUnknown {
    /// Start this task.
    pub unsafe fn Start(&self) -> HRESULT;

    /// Alert this task.
    pub unsafe fn Alert(&self) -> HRESULT;

    /// Join this task (wait for completion).
    pub unsafe fn Join(
        &self,
        dwMilliseconds: u32,
        option: u32,
    ) -> HRESULT;

    /// Set priority.
    pub unsafe fn SetPriority(&self, newPriority: i32) -> HRESULT;

    /// Get priority.
    pub unsafe fn GetPriority(&self, pPriority: *mut i32) -> HRESULT;

    /// Set CLR task.
    pub unsafe fn SetCLRTask(&self, pCLRTask: *mut ICLRTask) -> HRESULT;
}

/// ICLRTaskManager - Manages CLR tasks.
#[interface("4862efbe-3ae5-44f8-8feb-346190ee8a34")]
pub unsafe trait ICLRTaskManager: IUnknown {
    /// Create a CLR task.
    pub unsafe fn CreateTask(&self, ppTask: *mut *mut ICLRTask) -> HRESULT;

    /// Get current task.
    pub unsafe fn GetCurrentTask(&self, ppTask: *mut *mut ICLRTask) -> HRESULT;

    /// Set locale.
    pub unsafe fn SetLocale(&self, lcid: u32) -> HRESULT;

    /// Set UI locale.
    pub unsafe fn SetUILocale(&self, lcid: u32) -> HRESULT;

    /// Get current task type.
    pub unsafe fn GetCurrentTaskType(&self, pTaskType: *mut u32) -> HRESULT;
}

/// IHostTaskManager - Host-implemented task manager.
#[interface("997FF24C-43B7-4352-8667-0DC04FAFD354")]
pub unsafe trait IHostTaskManager: IUnknown {
    /// Get current task.
    pub unsafe fn GetCurrentTask(&self, pTask: *mut *mut IHostTask) -> HRESULT;

    /// Create task.
    pub unsafe fn CreateTask(
        &self,
        dwStackSize: usize,
        pStartAddress: *const c_void,
        pParameter: *mut c_void,
        ppTask: *mut *mut IHostTask,
    ) -> HRESULT;

    /// Sleep task.
    pub unsafe fn Sleep(&self, dwMilliseconds: u32, option: u32) -> HRESULT;

    /// Switch to task.
    pub unsafe fn SwitchToTask(&self, option: u32) -> HRESULT;

    /// Set locale.
    pub unsafe fn SetLocale(&self, lcid: u32) -> HRESULT;

    /// Set UI locale.
    pub unsafe fn SetUILocale(&self, lcid: u32) -> HRESULT;
}

