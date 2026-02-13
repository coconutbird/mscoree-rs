//! GUIDs for CLR hosting interfaces and classes.

use windows::core::GUID;

// CLSIDs (Class IDs)

/// CLSID for CLRMetaHost - use with CLRCreateInstance to get ICLRMetaHost
pub const CLSID_CLRMetaHost: GUID = GUID::from_u128(0x9280188d_0e8e_4867_b30c_7fa83884e8de);

/// CLSID for CLRMetaHostPolicy - use with CLRCreateInstance to get ICLRMetaHostPolicy  
pub const CLSID_CLRMetaHostPolicy: GUID = GUID::from_u128(0x2ebcd49a_1b47_4a61_b13a_4a03701e594b);

/// CLSID for CLRDebugging - use with CLRCreateInstance to get ICLRDebugging
pub const CLSID_CLRDebugging: GUID = GUID::from_u128(0xbacc578d_fbdd_48a4_969f_02d932b74634);

/// CLSID for CorRuntimeHost (legacy .NET 1.x hosting)
pub const CLSID_CorRuntimeHost: GUID = GUID::from_u128(0xcb2f6723_ab3a_11d2_9c40_00c04fa30a3e);

/// CLSID for CLRRuntimeHost (.NET 2.0+ hosting)
pub const CLSID_CLRRuntimeHost: GUID = GUID::from_u128(0x90f1a06e_7712_4762_86b5_7a5eba6bdb02);

// IIDs (Interface IDs)

/// IID for ICLRMetaHost interface
pub const IID_ICLRMetaHost: GUID = GUID::from_u128(0xd332db9e_b9b3_4125_8207_a14884f53216);

/// IID for ICLRMetaHostPolicy interface
pub const IID_ICLRMetaHostPolicy: GUID = GUID::from_u128(0xe2190695_77b2_492e_8e14_c4b3a7fdd593);

/// IID for ICLRRuntimeInfo interface
pub const IID_ICLRRuntimeInfo: GUID = GUID::from_u128(0xbd39d1d2_ba2f_486a_89b0_b4b0cb466891);

/// IID for ICLRRuntimeHost interface
pub const IID_ICLRRuntimeHost: GUID = GUID::from_u128(0x90f1a06c_7712_4762_86b5_7a5eba6bdb02);

/// IID for ICorRuntimeHost interface (legacy)
pub const IID_ICorRuntimeHost: GUID = GUID::from_u128(0xcb2f6722_ab3a_11d2_9c40_00c04fa30a3e);

/// IID for ICLRControl interface
pub const IID_ICLRControl: GUID = GUID::from_u128(0x9065597e_d1a1_4fb2_b6ba_7e1fce230f61);

/// IID for IHostControl interface
pub const IID_IHostControl: GUID = GUID::from_u128(0x02ca073c_7079_4860_880a_c2f7a449c991);

// Debugging Interface IIDs

/// IID for ICLRDataTarget interface
pub const IID_ICLRDataTarget: GUID = GUID::from_u128(0x3e11ccee_d08b_43e5_af01_32717a64da03);

/// IID for ICLRDataTarget2 interface
pub const IID_ICLRDataTarget2: GUID = GUID::from_u128(0x6d05fae3_189c_4630_a6dc_1c251e1c01ab);

/// IID for ICLRDataTarget3 interface
pub const IID_ICLRDataTarget3: GUID = GUID::from_u128(0xa5664f95_0af4_4a1b_960e_2f3346b4214c);

/// IID for IXCLRDataProcess interface
pub const IID_IXCLRDataProcess: GUID = GUID::from_u128(0x5c552ab6_fc09_4cb3_8e36_22fa03c798b7);

/// IID for IXCLRDataProcess2 interface
pub const IID_IXCLRDataProcess2: GUID = GUID::from_u128(0x5c552ab6_fc09_4cb3_8e36_22fa03c798b8);

/// IID for IXCLRDataTask interface
pub const IID_IXCLRDataTask: GUID = GUID::from_u128(0xa5b0beea_ec62_4618_8012_a24ffc23934c);

/// IID for IXCLRDataAppDomain interface
pub const IID_IXCLRDataAppDomain: GUID = GUID::from_u128(0x7ca04601_c702_4670_a63c_fa44f7da7bd5);

/// IID for IXCLRDataAssembly interface
pub const IID_IXCLRDataAssembly: GUID = GUID::from_u128(0x2fa17588_43c2_46ab_9b51_c8f01e39c9ac);

/// IID for IXCLRDataModule interface
pub const IID_IXCLRDataModule: GUID = GUID::from_u128(0x88e32849_0a0a_4cb0_9022_7cd2e9e139e2);

/// IID for IXCLRDataMethodInstance interface
pub const IID_IXCLRDataMethodInstance: GUID = GUID::from_u128(0xecd73800_22ca_4b0d_ab55_e9ba7e6318a5);

/// IID for IXCLRDataMethodDefinition interface
pub const IID_IXCLRDataMethodDefinition: GUID = GUID::from_u128(0xaaf60008_fb2c_420b_8fb1_42d244a54a97);

/// IID for IXCLRDataValue interface
pub const IID_IXCLRDataValue: GUID = GUID::from_u128(0x96ec93c7_1000_4e93_8991_98d8766e6666);

/// IID for IXCLRDataExceptionState interface
pub const IID_IXCLRDataExceptionState: GUID = GUID::from_u128(0x75da9e4c_bd33_43c8_8f5c_96e8a5241f57);

/// IID for IXCLRDataExceptionNotification interface
pub const IID_IXCLRDataExceptionNotification: GUID = GUID::from_u128(0x2d95a079_42a1_4837_818f_0b97d7048e0e);

/// IID for IXCLRDataTypeInstance interface
pub const IID_IXCLRDataTypeInstance: GUID = GUID::from_u128(0x4d078d91_b8ff_4d18_aeee_f24c881f7384);
