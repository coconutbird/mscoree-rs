//! ICLRDataTarget interface definitions for CLR debugging.
//!
//! These interfaces provide methods for interaction with a target item of the CLR.
//! They are used by debuggers and data access services.

#![allow(non_camel_case_types)]

use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// Type alias for CLRDATA_ADDRESS (64-bit virtual address)
pub type CLRDATA_ADDRESS = u64;

/// ICLRDataTarget interface for CLR debugging (.NET 2.0+).
///
/// Provides methods for interaction with a target item of the CLR.
/// The API client (debugger) must implement this interface as appropriate
/// for the particular target item.
#[interface("3E11CCEE-D08B-43E5-AF01-32717A64DA03")]
pub unsafe trait ICLRDataTarget: IUnknown {
    /// Gets the identifier for the kind of instruction set that the target process is using.
    ///
    /// # Arguments
    ///
    /// * `machineType` - Receives one of the IMAGE_FILE_MACHINE constants from WinNT.h
    pub unsafe fn GetMachineType(&self, machineType: *mut u32) -> HRESULT;

    /// Gets the size, in bytes, of a pointer to the current target.
    ///
    /// # Arguments
    ///
    /// * `pointerSize` - Receives the pointer size (4 for 32-bit, 8 for 64-bit)
    pub unsafe fn GetPointerSize(&self, pointerSize: *mut u32) -> HRESULT;

    /// Gets the base memory address for the specified image.
    ///
    /// # Arguments
    ///
    /// * `imagePath` - Path to the image
    /// * `baseAddress` - Receives the base address of the image
    pub unsafe fn GetImageBase(
        &self,
        imagePath: *const u16,
        baseAddress: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Reads data from the specified virtual memory address into the specified buffer.
    ///
    /// # Arguments
    ///
    /// * `address` - Virtual memory address to read from
    /// * `buffer` - Buffer to receive the data
    /// * `bytesRequested` - Number of bytes to read
    /// * `bytesRead` - Receives the number of bytes actually read
    pub unsafe fn ReadVirtual(
        &self,
        address: CLRDATA_ADDRESS,
        buffer: *mut u8,
        bytesRequested: u32,
        bytesRead: *mut u32,
    ) -> HRESULT;

    /// Writes data from the specified buffer to the specified virtual memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - Virtual memory address to write to
    /// * `buffer` - Buffer containing the data to write
    /// * `bytesRequested` - Number of bytes to write
    /// * `bytesWritten` - Receives the number of bytes actually written
    pub unsafe fn WriteVirtual(
        &self,
        address: CLRDATA_ADDRESS,
        buffer: *const u8,
        bytesRequested: u32,
        bytesWritten: *mut u32,
    ) -> HRESULT;

    /// Gets a value in thread local storage (TLS) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Operating system identifier of the thread
    /// * `index` - TLS index
    /// * `value` - Receives the TLS value
    pub unsafe fn GetTLSValue(
        &self,
        threadID: u32,
        index: u32,
        value: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Sets a value in thread local storage (TLS) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Operating system identifier of the thread
    /// * `index` - TLS index
    /// * `value` - The TLS value to set
    pub unsafe fn SetTLSValue(
        &self,
        threadID: u32,
        index: u32,
        value: CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Gets the operating system identifier for the current thread.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Receives the thread ID
    pub unsafe fn GetCurrentThreadID(&self, threadID: *mut u32) -> HRESULT;

    /// Gets the execution context for the given thread.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Operating system identifier of the thread
    /// * `contextFlags` - Flags specifying which parts of the context to return
    /// * `contextSize` - Size of the context buffer
    /// * `context` - Buffer to receive the CONTEXT structure
    pub unsafe fn GetThreadContext(
        &self,
        threadID: u32,
        contextFlags: u32,
        contextSize: u32,
        context: *mut u8,
    ) -> HRESULT;

    /// Sets the current context of the specified thread.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Operating system identifier of the thread
    /// * `contextSize` - Size of the context buffer
    /// * `context` - Buffer containing the CONTEXT structure
    pub unsafe fn SetThreadContext(
        &self,
        threadID: u32,
        contextSize: u32,
        context: *const u8,
    ) -> HRESULT;

    /// Called by CLR data access services to request an operation.
    ///
    /// # Arguments
    ///
    /// * `reqCode` - Request code defining the operation
    /// * `inBufferSize` - Size of the input buffer
    /// * `inBuffer` - Input buffer
    /// * `outBufferSize` - Size of the output buffer
    /// * `outBuffer` - Output buffer
    pub unsafe fn Request(
        &self,
        reqCode: u32,
        inBufferSize: u32,
        inBuffer: *const u8,
        outBufferSize: u32,
        outBuffer: *mut u8,
    ) -> HRESULT;
}

/// ICLRDataTarget2 interface for CLR debugging (.NET 2.0+).
///
/// A subclass of ICLRDataTarget that provides methods to manipulate
/// virtual memory regions in the target process.
#[interface("6D05FAE3-189C-4630-A6DC-1C251E1C01AB")]
pub unsafe trait ICLRDataTarget2: ICLRDataTarget {
    /// Allocates memory in the address space of the target process.
    ///
    /// # Arguments
    ///
    /// * `addr` - Specifies the requested starting address (or 0 for system choice)
    /// * `size` - Size of the region to allocate
    /// * `typeFlags` - Memory allocation type flags (MEM_COMMIT, MEM_RESERVE, etc.)
    /// * `protectFlags` - Memory protection flags (PAGE_EXECUTE_READWRITE, etc.)
    /// * `virt` - Receives the actual starting address of the allocated region
    pub unsafe fn AllocVirtual(
        &self,
        addr: CLRDATA_ADDRESS,
        size: u32,
        typeFlags: u32,
        protectFlags: u32,
        virt: *mut CLRDATA_ADDRESS,
    ) -> HRESULT;

    /// Frees memory that was previously allocated in the target process.
    ///
    /// # Arguments
    ///
    /// * `addr` - Starting address of the region to free
    /// * `size` - Size of the region to free
    /// * `typeFlags` - Memory free type flags (MEM_DECOMMIT, MEM_RELEASE)
    pub unsafe fn FreeVirtual(
        &self,
        addr: CLRDATA_ADDRESS,
        size: u32,
        typeFlags: u32,
    ) -> HRESULT;
}

/// ICLRDataTarget3 interface for CLR debugging (.NET 4.5.1+).
///
/// A subclass of ICLRDataTarget2 that provides access to exception information.
#[interface("A5664F95-0AF4-4A1B-960E-2F3346B4214C")]
pub unsafe trait ICLRDataTarget3: ICLRDataTarget2 {
    /// Gets the exception record associated with the target process.
    ///
    /// # Arguments
    ///
    /// * `bufferSize` - Size of the buffer
    /// * `bufferUsed` - Receives the number of bytes written
    /// * `buffer` - Buffer to receive the exception record
    pub unsafe fn GetExceptionRecord(
        &self,
        bufferSize: u32,
        bufferUsed: *mut u32,
        buffer: *mut u8,
    ) -> HRESULT;

    /// Gets the context record associated with the target process.
    ///
    /// # Arguments
    ///
    /// * `bufferSize` - Size of the buffer
    /// * `bufferUsed` - Receives the number of bytes written
    /// * `buffer` - Buffer to receive the context record
    pub unsafe fn GetExceptionContextRecord(
        &self,
        bufferSize: u32,
        bufferUsed: *mut u32,
        buffer: *mut u8,
    ) -> HRESULT;

    /// Gets the ID of the thread that threw the exception.
    ///
    /// # Arguments
    ///
    /// * `threadID` - Receives the thread ID
    pub unsafe fn GetExceptionThreadID(&self, threadID: *mut u32) -> HRESULT;
}

