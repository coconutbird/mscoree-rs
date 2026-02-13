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
