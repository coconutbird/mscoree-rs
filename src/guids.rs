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

// SOS DAC Interface IIDs

/// IID for ISOSEnum interface
pub const IID_ISOSEnum: GUID = GUID::from_u128(0x286CA186_E763_4F61_9760_487D43AE4341);

/// IID for ISOSHandleEnum interface
pub const IID_ISOSHandleEnum: GUID = GUID::from_u128(0x3E269830_4A2B_4301_8EE2_D6805B29B2FA);

/// IID for ISOSStackRefErrorEnum interface
pub const IID_ISOSStackRefErrorEnum: GUID = GUID::from_u128(0x774F4E1B_FB7B_491B_976D_A8130FE355E9);

/// IID for ISOSStackRefEnum interface
pub const IID_ISOSStackRefEnum: GUID = GUID::from_u128(0x8FA642BD_9F10_4799_9AA3_512AE78C77EE);

/// IID for ISOSMemoryEnum interface
pub const IID_ISOSMemoryEnum: GUID = GUID::from_u128(0xE4B860EC_337A_40C0_A591_F09A9680690F);

/// IID for ISOSMethodEnum interface
pub const IID_ISOSMethodEnum: GUID = GUID::from_u128(0x3C0FE725_C324_4A4F_8100_D399588A662E);

/// IID for ISOSDacInterface interface
pub const IID_ISOSDacInterface: GUID = GUID::from_u128(0x436f00f2_b42a_4b9f_870c_e73db66ae930);

/// IID for ISOSDacInterface2 interface
pub const IID_ISOSDacInterface2: GUID = GUID::from_u128(0xA16026EC_96F4_40BA_87FB_5575986FB7AF);

/// IID for ISOSDacInterface3 interface
pub const IID_ISOSDacInterface3: GUID = GUID::from_u128(0xB08C5CDC_FD8A_49C5_AB38_5FEEF35235B4);

/// IID for ISOSDacInterface4 interface
pub const IID_ISOSDacInterface4: GUID = GUID::from_u128(0x74B9D34C_A612_4B07_93DD_5462178FCE11);

/// IID for ISOSDacInterface5 interface
pub const IID_ISOSDacInterface5: GUID = GUID::from_u128(0x127d6abe_6c86_4e48_8e7b_220781c58101);

/// IID for ISOSDacInterface6 interface
pub const IID_ISOSDacInterface6: GUID = GUID::from_u128(0x11206399_4B66_4EDB_98EA_85654E59AD45);

/// IID for ISOSDacInterface7 interface
pub const IID_ISOSDacInterface7: GUID = GUID::from_u128(0xc1020dde_fe98_4536_a53b_f35a74c327eb);

/// IID for ISOSDacInterface8 interface
pub const IID_ISOSDacInterface8: GUID = GUID::from_u128(0xc12f35a9_e55c_4520_a894_b3dc5165dfce);

/// IID for ISOSDacInterface9 interface
pub const IID_ISOSDacInterface9: GUID = GUID::from_u128(0x4eca42d8_7e7b_4c8a_a116_7bfbf6929267);

/// IID for ISOSDacInterface10 interface
pub const IID_ISOSDacInterface10: GUID = GUID::from_u128(0x90B8FCC3_7251_4B0A_AE3D_5C13A67EC9AA);

/// IID for ISOSDacInterface11 interface
pub const IID_ISOSDacInterface11: GUID = GUID::from_u128(0x96BA1DB9_14CD_4492_8065_1CAAECF6E5CF);

/// IID for ISOSDacInterface12 interface
pub const IID_ISOSDacInterface12: GUID = GUID::from_u128(0x1b93bacc_8ca4_432d_943a_3e6e7ec0b0a3);

/// IID for ISOSDacInterface13 interface
pub const IID_ISOSDacInterface13: GUID = GUID::from_u128(0x3176a8ed_597b_4f54_a71f_83695c6a8c5e);

/// IID for ISOSDacInterface14 interface
pub const IID_ISOSDacInterface14: GUID = GUID::from_u128(0x9aa22aca_6dc6_4a0c_b4e0_70d2416b9837);

/// IID for ISOSDacInterface15 interface
pub const IID_ISOSDacInterface15: GUID = GUID::from_u128(0x7ed81261_52a9_4a23_a358_c3313dea30a8);

/// IID for ISOSDacInterface16 interface
pub const IID_ISOSDacInterface16: GUID = GUID::from_u128(0x4ba12ff8_daac_4e43_ac56_98cf8d5c595d);
