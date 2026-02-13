//! XCLR Data types, flags, and enumerations.
//!
//! These types are used by the IXCLRData* interfaces for CLR debugging.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::data_target::CLRDATA_ADDRESS;

/// CLR data process flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataProcessFlag {
    CLRDATA_PROCESS_DEFAULT = 0x00000000,
    CLRDATA_PROCESS_IN_GC = 0x00000001,
}

/// CLR data app domain flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataAppDomainFlag {
    CLRDATA_DOMAIN_DEFAULT = 0x00000000,
}

/// CLR data task flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataTaskFlag {
    CLRDATA_TASK_DEFAULT = 0x00000000,
    CLRDATA_TASK_WAITING_FOR_GC = 0x00000001,
}

/// CLR data type flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataTypeFlag {
    CLRDATA_TYPE_DEFAULT = 0x00000000,
    CLRDATA_TYPE_IS_PRIMITIVE = 0x00000001,
    CLRDATA_TYPE_IS_VALUE_TYPE = 0x00000002,
    CLRDATA_TYPE_IS_STRING = 0x00000004,
    CLRDATA_TYPE_IS_ARRAY = 0x00000008,
    CLRDATA_TYPE_IS_REFERENCE = 0x00000010,
    CLRDATA_TYPE_IS_POINTER = 0x00000020,
    CLRDATA_TYPE_IS_ENUM = 0x00000040,
    CLRDATA_TYPE_ALL_KINDS = 0x7f,
}

/// CLR data field flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataFieldFlag {
    CLRDATA_FIELD_DEFAULT = 0x00000000,
    CLRDATA_FIELD_IS_PRIMITIVE = 0x00000001,
    CLRDATA_FIELD_IS_VALUE_TYPE = 0x00000002,
    CLRDATA_FIELD_IS_STRING = 0x00000004,
    CLRDATA_FIELD_IS_ARRAY = 0x00000008,
    CLRDATA_FIELD_IS_REFERENCE = 0x00000010,
    CLRDATA_FIELD_IS_POINTER = 0x00000020,
    CLRDATA_FIELD_IS_ENUM = 0x00000040,
    CLRDATA_FIELD_ALL_KINDS = 0x7f,
    CLRDATA_FIELD_IS_INHERITED = 0x00000080,
    CLRDATA_FIELD_IS_LITERAL = 0x00000100,
    CLRDATA_FIELD_FROM_INSTANCE = 0x00000200,
    CLRDATA_FIELD_FROM_TASK_LOCAL = 0x00000400,
    CLRDATA_FIELD_FROM_STATIC = 0x00000800,
    CLRDATA_FIELD_ALL_LOCATIONS = 0x00000e00,
    CLRDATA_FIELD_ALL_FIELDS = 0x00000eff,
}

/// CLR data value flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataValueFlag {
    CLRDATA_VALUE_DEFAULT = 0x00000000,
    CLRDATA_VALUE_IS_PRIMITIVE = 0x00000001,
    CLRDATA_VALUE_IS_VALUE_TYPE = 0x00000002,
    CLRDATA_VALUE_IS_STRING = 0x00000004,
    CLRDATA_VALUE_IS_ARRAY = 0x00000008,
    CLRDATA_VALUE_IS_REFERENCE = 0x00000010,
    CLRDATA_VALUE_IS_POINTER = 0x00000020,
    CLRDATA_VALUE_IS_ENUM = 0x00000040,
    CLRDATA_VALUE_ALL_KINDS = 0x7f,
    CLRDATA_VALUE_IS_INHERITED = 0x00000080,
    CLRDATA_VALUE_IS_LITERAL = 0x00000100,
    CLRDATA_VALUE_FROM_INSTANCE = 0x00000200,
    CLRDATA_VALUE_FROM_TASK_LOCAL = 0x00000400,
    CLRDATA_VALUE_FROM_STATIC = 0x00000800,
    CLRDATA_VALUE_ALL_LOCATIONS = 0x00000e00,
    CLRDATA_VALUE_ALL_FIELDS = 0x00000eff,
    CLRDATA_VALUE_IS_BOXED = 0x00001000,
}

/// CLR data by-name flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataByNameFlag {
    CLRDATA_BYNAME_CASE_SENSITIVE = 0x00000000,
    CLRDATA_BYNAME_CASE_INSENSITIVE = 0x00000001,
}

/// CLR data get-name flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataGetNameFlag {
    CLRDATA_GETNAME_DEFAULT = 0x00000000,
    CLRDATA_GETNAME_NO_NAMESPACES = 0x00000001,
    CLRDATA_GETNAME_NO_PARAMETERS = 0x00000002,
}

/// Simple frame type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataSimpleFrameType {
    CLRDATA_SIMPFRAME_UNRECOGNIZED = 0x1,
    CLRDATA_SIMPFRAME_MANAGED_METHOD = 0x2,
    CLRDATA_SIMPFRAME_RUNTIME_MANAGED_CODE = 0x4,
    CLRDATA_SIMPFRAME_RUNTIME_UNMANAGED_CODE = 0x8,
}

/// Detailed frame type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataDetailedFrameType {
    CLRDATA_DETFRAME_UNRECOGNIZED = 0,
    CLRDATA_DETFRAME_UNKNOWN_STUB = 1,
    CLRDATA_DETFRAME_CLASS_INIT = 2,
    CLRDATA_DETFRAME_EXCEPTION_FILTER = 3,
    CLRDATA_DETFRAME_SECURITY = 4,
    CLRDATA_DETFRAME_CONTEXT_POLICY = 5,
    CLRDATA_DETFRAME_INTERCEPTION = 6,
    CLRDATA_DETFRAME_PROCESS_START = 7,
    CLRDATA_DETFRAME_THREAD_START = 8,
    CLRDATA_DETFRAME_TRANSITION_TO_MANAGED = 9,
    CLRDATA_DETFRAME_TRANSITION_TO_UNMANAGED = 10,
    CLRDATA_DETFRAME_COM_INTEROP_STUB = 11,
    CLRDATA_DETFRAME_DEBUGGER_EVAL = 12,
    CLRDATA_DETFRAME_CONTEXT_SWITCH = 13,
    CLRDATA_DETFRAME_FUNC_EVAL = 14,
    CLRDATA_DETFRAME_FINALLY = 15,
}

/// Method extent types.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataMethodExtentType {
    CYCLER_DATA_METHEXT_UNKNOWN = 0x00000000,
    CLRDATA_METHEXT_IL = 0x00000001,
    CLRDATA_METHEXT_NATIVE_CODE = 0x00000002,
    CLRDATA_METHEXT_NATIVE_COLD_CODE = 0x00000004,
}

/// Module flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataModuleFlag {
    CLRDATA_MODULE_DEFAULT = 0x00000000,
    CLRDATA_MODULE_IS_DYNAMIC = 0x00000001,
    CLRDATA_MODULE_IS_MEMORY = 0x00000002,
    CLRDATA_MODULE_IS_MAIN_MODULE = 0x00000004,
}

/// Assembly flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataAssemblyFlag {
    CLRDATA_ASSEMBLY_DEFAULT = 0x00000000,
}

/// Stack walk context flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataStackWalkContext {
    CLRDATA_STACK_SET_UNWIND_CONTEXT = 0x00000000,
    CLRDATA_STACK_SET_CURRENT_CONTEXT = 0x00000001,
}

/// Exception state flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataExceptionStateFlag {
    CYCLER_DATA_EXCEPTION_DEFAULT = 0x00000000,
    CLRDATA_EXCEPTION_PARTIAL = 0x00000001,
    CLRDATA_EXCEPTION_NESTED = 0x00000002,
}

/// Exception notification flags.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataExceptionNotificationFlag {
    CYCLER_DATA_NOTIFY_ON_MODULE_LOAD = 0x00000001,
    CLRDATA_NOTIFY_ON_MODULE_UNLOAD = 0x00000002,
    CLRDATA_NOTIFY_ON_EXCEPTION = 0x00000004,
    CLRDATA_NOTIFY_ON_EXCEPTION_CATCH_ENTER = 0x00000008,
}

/// Source type for method extent.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLRDataSourceType {
    CLRDATA_SOURCE_TYPE_UNDEFINED = 0,
}

/// Method definition extent structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CLRDATA_METHDEF_EXTENT {
    pub startAddress: CLRDATA_ADDRESS,
    pub endAddress: CLRDATA_ADDRESS,
    pub enCVersion: u32,
    pub r#type: CLRDataMethodExtentType,
}

/// IL to address map entry.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CLRDATA_IL_ADDRESS_MAP {
    pub ilOffset: u32,
    pub startAddress: CLRDATA_ADDRESS,
    pub endAddress: CLRDATA_ADDRESS,
    pub r#type: CLRDataMethodExtentType,
}

/// GC event types.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcEvt_t {
    GC_MARK_END = 1,
}

/// GC event arguments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GcEvtArgs {
    pub typ: GcEvt_t,
    pub condemnedGeneration: i32,
}
