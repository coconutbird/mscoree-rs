//! IXCLRDataExceptionState and notification interfaces.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

use super::clr_data_target::CLRDATA_ADDRESS;

/// IXCLRDataExceptionState - Represents exception state.
#[interface("75DA9E4C-BD33-43C8-8F5C-96E8A5241F57")]
pub unsafe trait IXCLRDataExceptionState: IUnknown {
    /// Get the flags for this exception state.
    pub unsafe fn GetFlags(&self, flags: *mut u32) -> HRESULT;

    /// Get the previous exception in the chain.
    pub unsafe fn GetPrevious(&self, exState: *mut *mut IXCLRDataExceptionState) -> HRESULT;

    /// Get the managed exception object.
    pub unsafe fn GetManagedObject(&self, value: *mut *mut IUnknown) -> HRESULT;

    /// Get the base type of the exception.
    pub unsafe fn GetBaseType(&self, r#type: *mut u32) -> HRESULT;

    /// Get the exception code.
    pub unsafe fn GetCode(&self, code: *mut u32) -> HRESULT;

    /// Get the exception string.
    pub unsafe fn GetString(&self, bufLen: u32, strLen: *mut u32, str: *mut u16) -> HRESULT;

    /// Generic request operation.
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;

    /// Determine whether this is the same state as another.
    pub unsafe fn IsSameState(&self, exState: *mut IXCLRDataExceptionState) -> HRESULT;

    /// Determine whether this is the same state using record pointer.
    pub unsafe fn IsSameState2(&self, flags: u32, exRecord: *const u8) -> HRESULT;

    /// Get the task for this exception.
    pub unsafe fn GetTask(&self, task: *mut *mut IUnknown) -> HRESULT;
}

/// IXCLRDataExceptionNotification - Exception notification callback.
#[interface("2D95A079-42A1-4837-818F-0B97D7048E0E")]
pub unsafe trait IXCLRDataExceptionNotification: IUnknown {
    /// Called when a new task is created.
    pub unsafe fn OnTaskCreated(&self, task: *mut IUnknown) -> HRESULT;

    /// Called when a task is destroyed.
    pub unsafe fn OnTaskDestroyed(&self, task: *mut IUnknown) -> HRESULT;

    /// Called when a module is loaded.
    pub unsafe fn OnModuleLoaded(&self, module: *mut IUnknown) -> HRESULT;

    /// Called when a module is unloaded.
    pub unsafe fn OnModuleUnloaded(&self, module: *mut IUnknown) -> HRESULT;

    /// Called when a type is loaded.
    pub unsafe fn OnTypeLoaded(&self, typeInstance: *mut IUnknown) -> HRESULT;

    /// Called when a type is unloaded.
    pub unsafe fn OnTypeUnloaded(&self, typeInstance: *mut IUnknown) -> HRESULT;
}

/// IXCLRDataExceptionNotification2 - Extended exception notification.
#[interface("31201a94-4337-49b7-aef7-0c7550f0f3af")]
pub unsafe trait IXCLRDataExceptionNotification2: IXCLRDataExceptionNotification {
    /// Called when an app domain is loaded.
    pub unsafe fn OnAppDomainLoaded(&self, domain: *mut IUnknown) -> HRESULT;

    /// Called when an app domain is unloaded.
    pub unsafe fn OnAppDomainUnloaded(&self, domain: *mut IUnknown) -> HRESULT;

    /// Called when an exception is thrown.
    pub unsafe fn OnException(&self, exception: *mut IXCLRDataExceptionState) -> HRESULT;
}

/// IXCLRDataExceptionNotification3 - More extended exception notification.
#[interface("31201a94-4337-49b7-aef7-0c7550000003")]
pub unsafe trait IXCLRDataExceptionNotification3: IXCLRDataExceptionNotification2 {
    /// Called when a GC event occurs.
    pub unsafe fn OnGcEvent(&self, gcEvent: u32) -> HRESULT;
}

/// IXCLRDataExceptionNotification4 - More extended exception notification.
#[interface("C25E926E-5F09-4AA2-BBAD-B7FC7F10CFD7")]
pub unsafe trait IXCLRDataExceptionNotification4: IXCLRDataExceptionNotification3 {
    /// Called for exception catch entry.
    pub unsafe fn ExceptionCatcherEnter(
        &self,
        methodInstance: *mut IUnknown,
        catcherNativeOffset: u32,
    ) -> HRESULT;
}

/// IXCLRDataExceptionNotification5 - Most extended exception notification.
#[interface("e77a39ea-3548-44d9-b171-8569ed1a9423")]
pub unsafe trait IXCLRDataExceptionNotification5: IXCLRDataExceptionNotification4 {
    /// Called for concurrency notification.
    pub unsafe fn OnCodeGenerated2(
        &self,
        method: *mut IUnknown,
        nativeCodeLocation: CLRDATA_ADDRESS,
    ) -> HRESULT;
}
