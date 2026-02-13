//! IMetaDataImport interface definitions for reading assembly metadata.

use std::ffi::c_void;
use windows::core::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, interface};

/// IMetaDataImport - Read metadata from an assembly.
#[interface("7DAC8207-D3AE-4C75-9B67-92801A497D44")]
pub unsafe trait IMetaDataImport: IUnknown {
    /// Close an enumeration.
    pub unsafe fn CloseEnum(&self, hEnum: *mut c_void) -> HRESULT;

    /// Count the number of tokens in an enumeration.
    pub unsafe fn CountEnum(&self, hEnum: *mut c_void, pulCount: *mut u32) -> HRESULT;

    /// Reset an enumeration.
    pub unsafe fn ResetEnum(&self, hEnum: *mut c_void, ulPos: u32) -> HRESULT;

    /// Enumerate type definitions.
    pub unsafe fn EnumTypeDefs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeDefs: *mut u32,
        cMax: u32,
        pcTypeDefs: *mut u32,
    ) -> HRESULT;

    /// Enumerate interface implementations.
    pub unsafe fn EnumInterfaceImpls(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rImpls: *mut u32,
        cMax: u32,
        pcImpls: *mut u32,
    ) -> HRESULT;

    /// Enumerate type references.
    pub unsafe fn EnumTypeRefs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeRefs: *mut u32,
        cMax: u32,
        pcTypeRefs: *mut u32,
    ) -> HRESULT;

    /// Find a type definition by name.
    pub unsafe fn FindTypeDefByName(
        &self,
        szTypeDef: *const u16,
        tkEnclosingClass: u32,
        ptd: *mut u32,
    ) -> HRESULT;

    /// Get the scope properties.
    pub unsafe fn GetScopeProps(
        &self,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pmvid: *mut GUID,
    ) -> HRESULT;

    /// Get the module from a scope.
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut u32) -> HRESULT;

    /// Get type definition properties.
    pub unsafe fn GetTypeDefProps(
        &self,
        td: u32,
        szTypeDef: *mut u16,
        cchTypeDef: u32,
        pchTypeDef: *mut u32,
        pdwTypeDefFlags: *mut u32,
        ptkExtends: *mut u32,
    ) -> HRESULT;

    /// Get interface implementation properties.
    pub unsafe fn GetInterfaceImplProps(
        &self,
        iiImpl: u32,
        pClass: *mut u32,
        ptkIface: *mut u32,
    ) -> HRESULT;

    /// Get type reference properties.
    pub unsafe fn GetTypeRefProps(
        &self,
        tr: u32,
        ptkResolutionScope: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
    ) -> HRESULT;

    /// Resolve a type reference.
    pub unsafe fn ResolveTypeRef(
        &self,
        tr: u32,
        riid: *const GUID,
        ppIScope: *mut *mut IUnknown,
        ptd: *mut u32,
    ) -> HRESULT;

    /// Enumerate members.
    pub unsafe fn EnumMembers(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rMembers: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate members with a specific name.
    pub unsafe fn EnumMembersWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rMembers: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate methods.
    pub unsafe fn EnumMethods(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate methods with a specific name.
    pub unsafe fn EnumMethodsWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate fields.
    pub unsafe fn EnumFields(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rFields: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate fields with a specific name.
    pub unsafe fn EnumFieldsWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rFields: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate parameters.
    pub unsafe fn EnumParams(
        &self,
        phEnum: *mut *mut c_void,
        mb: u32,
        rParams: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate member references.
    pub unsafe fn EnumMemberRefs(
        &self,
        phEnum: *mut *mut c_void,
        tkParent: u32,
        rMemberRefs: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate method implementations.
    pub unsafe fn EnumMethodImpls(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rMethodBody: *mut u32,
        rMethodDecl: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Enumerate permissions sets.
    pub unsafe fn EnumPermissionSets(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        dwActions: u32,
        rPermission: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Find a member.
    pub unsafe fn FindMember(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;

    /// Find a method.
    pub unsafe fn FindMethod(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;

    /// Find a field.
    pub unsafe fn FindField(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;

    /// Find a member reference.
    pub unsafe fn FindMemberRef(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmr: *mut u32,
    ) -> HRESULT;

    /// Get method properties.
    pub unsafe fn GetMethodProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szMethod: *mut u16,
        cchMethod: u32,
        pchMethod: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pulCodeRVA: *mut u32,
        pdwImplFlags: *mut u32,
    ) -> HRESULT;

    /// Get member reference properties.
    pub unsafe fn GetMemberRefProps(
        &self,
        mr: u32,
        ptk: *mut u32,
        szMember: *mut u16,
        cchMember: u32,
        pchMember: *mut u32,
        ppvSigBlob: *mut *const u8,
        pbSig: *mut u32,
    ) -> HRESULT;

    /// Enumerate properties.
    pub unsafe fn EnumProperties(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rProperties: *mut u32,
        cMax: u32,
        pcProperties: *mut u32,
    ) -> HRESULT;

    /// Enumerate events.
    pub unsafe fn EnumEvents(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rEvents: *mut u32,
        cMax: u32,
        pcEvents: *mut u32,
    ) -> HRESULT;

    /// Get event properties.
    pub unsafe fn GetEventProps(
        &self,
        ev: u32,
        pClass: *mut u32,
        szEvent: *mut u16,
        cchEvent: u32,
        pchEvent: *mut u32,
        pdwEventFlags: *mut u32,
        ptkEventType: *mut u32,
        pmdAddOn: *mut u32,
        pmdRemoveOn: *mut u32,
        pmdFire: *mut u32,
        rmdOtherMethod: *mut u32,
        cMax: u32,
        pcOtherMethod: *mut u32,
    ) -> HRESULT;

    /// Enumerate method semantics.
    pub unsafe fn EnumMethodSemantics(
        &self,
        phEnum: *mut *mut c_void,
        mb: u32,
        rEventProp: *mut u32,
        cMax: u32,
        pcEventProp: *mut u32,
    ) -> HRESULT;

    /// Get method semantics.
    pub unsafe fn GetMethodSemantics(
        &self,
        mb: u32,
        tkEventProp: u32,
        pdwSemanticsFlags: *mut u32,
    ) -> HRESULT;

    /// Get class layout.
    pub unsafe fn GetClassLayout(
        &self,
        td: u32,
        pdwPackSize: *mut u32,
        rFieldOffset: *mut c_void,
        cMax: u32,
        pcFieldOffset: *mut u32,
        pulClassSize: *mut u32,
    ) -> HRESULT;

    /// Get field marshal.
    pub unsafe fn GetFieldMarshal(
        &self,
        tk: u32,
        ppvNativeType: *mut *const u8,
        pcbNativeType: *mut u32,
    ) -> HRESULT;

    /// Get RVA.
    pub unsafe fn GetRVA(&self, tk: u32, pulCodeRVA: *mut u32, pdwImplFlags: *mut u32) -> HRESULT;

    /// Get permission set properties.
    pub unsafe fn GetPermissionSetProps(
        &self,
        pm: u32,
        pdwAction: *mut u32,
        ppvPermission: *mut *const c_void,
        pcbPermission: *mut u32,
    ) -> HRESULT;

    /// Get signature from token.
    pub unsafe fn GetSigFromToken(
        &self,
        mdSig: u32,
        ppvSig: *mut *const u8,
        pcbSig: *mut u32,
    ) -> HRESULT;

    /// Get module reference properties.
    pub unsafe fn GetModuleRefProps(
        &self,
        mur: u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
    ) -> HRESULT;

    /// Enumerate module references.
    pub unsafe fn EnumModuleRefs(
        &self,
        phEnum: *mut *mut c_void,
        rModuleRefs: *mut u32,
        cmax: u32,
        pcModuleRefs: *mut u32,
    ) -> HRESULT;

    /// Get type spec from token.
    pub unsafe fn GetTypeSpecFromToken(
        &self,
        typespec: u32,
        ppvSig: *mut *const u8,
        pcbSig: *mut u32,
    ) -> HRESULT;

    /// Get the name from a token.
    pub unsafe fn GetNameFromToken(&self, tk: u32, pszUtf8NamePtr: *mut *const u8) -> HRESULT;

    /// Enumerate unresolved methods.
    pub unsafe fn EnumUnresolvedMethods(
        &self,
        phEnum: *mut *mut c_void,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;

    /// Get user string.
    pub unsafe fn GetUserString(
        &self,
        stk: u32,
        szString: *mut u16,
        cchString: u32,
        pchString: *mut u32,
    ) -> HRESULT;

    /// Get PInvoke map.
    pub unsafe fn GetPinvokeMap(
        &self,
        tk: u32,
        pdwMappingFlags: *mut u32,
        szImportName: *mut u16,
        cchImportName: u32,
        pchImportName: *mut u32,
        pmrImportDLL: *mut u32,
    ) -> HRESULT;

    /// Enumerate signatures.
    pub unsafe fn EnumSignatures(
        &self,
        phEnum: *mut *mut c_void,
        rSignatures: *mut u32,
        cmax: u32,
        pcSignatures: *mut u32,
    ) -> HRESULT;

    /// Enumerate type specs.
    pub unsafe fn EnumTypeSpecs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeSpecs: *mut u32,
        cmax: u32,
        pcTypeSpecs: *mut u32,
    ) -> HRESULT;

    /// Enumerate user strings.
    pub unsafe fn EnumUserStrings(
        &self,
        phEnum: *mut *mut c_void,
        rStrings: *mut u32,
        cmax: u32,
        pcStrings: *mut u32,
    ) -> HRESULT;

    /// Get parent token.
    pub unsafe fn GetParentToken(&self, tk: u32, ptk: *mut u32) -> HRESULT;

    /// Enumerate custom attributes.
    pub unsafe fn EnumCustomAttributes(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        tkType: u32,
        rCustomAttributes: *mut u32,
        cMax: u32,
        pcCustomAttributes: *mut u32,
    ) -> HRESULT;

    /// Get custom attribute properties.
    pub unsafe fn GetCustomAttributeProps(
        &self,
        cv: u32,
        ptkObj: *mut u32,
        ptkType: *mut u32,
        ppBlob: *mut *const c_void,
        pcbSize: *mut u32,
    ) -> HRESULT;

    /// Find type reference by name.
    pub unsafe fn FindTypeRef(
        &self,
        tkResolutionScope: u32,
        szName: *const u16,
        ptr: *mut u32,
    ) -> HRESULT;

    /// Get member properties.
    pub unsafe fn GetMemberProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szMember: *mut u16,
        cchMember: u32,
        pchMember: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pulCodeRVA: *mut u32,
        pdwImplFlags: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;

    /// Get field properties.
    pub unsafe fn GetFieldProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szField: *mut u16,
        cchField: u32,
        pchField: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;

    /// Get property properties.
    pub unsafe fn GetPropertyProps(
        &self,
        prop: u32,
        pClass: *mut u32,
        szProperty: *mut u16,
        cchProperty: u32,
        pchProperty: *mut u32,
        pdwPropFlags: *mut u32,
        ppvSig: *mut *const u8,
        pbSig: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppDefaultValue: *mut *const c_void,
        pcchDefaultValue: *mut u32,
        pmdSetter: *mut u32,
        pmdGetter: *mut u32,
        rmdOtherMethod: *mut u32,
        cMax: u32,
        pcOtherMethod: *mut u32,
    ) -> HRESULT;

    /// Get parameter properties.
    pub unsafe fn GetParamProps(
        &self,
        tk: u32,
        pmd: *mut u32,
        pulSequence: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pdwAttr: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;

    /// Get custom attribute by name.
    pub unsafe fn GetCustomAttributeByName(
        &self,
        tkObj: u32,
        szName: *const u16,
        ppData: *mut *const c_void,
        pcbData: *mut u32,
    ) -> HRESULT;

    /// Check if a name is a valid name.
    pub unsafe fn IsValidToken(&self, tk: u32) -> i32;

    /// Get the nested class encloser.
    pub unsafe fn GetNestedClassProps(
        &self,
        tdNestedClass: u32,
        ptdEnclosingClass: *mut u32,
    ) -> HRESULT;

    /// Get native call convention from sig.
    pub unsafe fn GetNativeCallConvFromSig(
        &self,
        pvSig: *const c_void,
        cbSig: u32,
        pCallConv: *mut u32,
    ) -> HRESULT;

    /// Is global.
    pub unsafe fn IsGlobal(&self, pd: u32, pbGlobal: *mut i32) -> HRESULT;
}

/// IMetaDataImport2 - Extended metadata import interface.
#[interface("FCE5EFA0-8BBA-4F8E-A036-8F2022B08466")]
pub unsafe trait IMetaDataImport2: IUnknown {
    // All IMetaDataImport methods first (inherited)
    pub unsafe fn CloseEnum(&self, hEnum: *mut c_void) -> HRESULT;
    pub unsafe fn CountEnum(&self, hEnum: *mut c_void, pulCount: *mut u32) -> HRESULT;
    pub unsafe fn ResetEnum(&self, hEnum: *mut c_void, ulPos: u32) -> HRESULT;
    pub unsafe fn EnumTypeDefs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeDefs: *mut u32,
        cMax: u32,
        pcTypeDefs: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumInterfaceImpls(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rImpls: *mut u32,
        cMax: u32,
        pcImpls: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumTypeRefs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeRefs: *mut u32,
        cMax: u32,
        pcTypeRefs: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindTypeDefByName(
        &self,
        szTypeDef: *const u16,
        tkEnclosingClass: u32,
        ptd: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetScopeProps(
        &self,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pmvid: *mut GUID,
    ) -> HRESULT;
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut u32) -> HRESULT;
    pub unsafe fn GetTypeDefProps(
        &self,
        td: u32,
        szTypeDef: *mut u16,
        cchTypeDef: u32,
        pchTypeDef: *mut u32,
        pdwTypeDefFlags: *mut u32,
        ptkExtends: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetInterfaceImplProps(
        &self,
        iiImpl: u32,
        pClass: *mut u32,
        ptkIface: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetTypeRefProps(
        &self,
        tr: u32,
        ptkResolutionScope: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
    ) -> HRESULT;
    pub unsafe fn ResolveTypeRef(
        &self,
        tr: u32,
        riid: *const GUID,
        ppIScope: *mut *mut IUnknown,
        ptd: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMembers(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rMembers: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMembersWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rMembers: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMethods(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMethodsWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumFields(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        rFields: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumFieldsWithName(
        &self,
        phEnum: *mut *mut c_void,
        cl: u32,
        szName: *const u16,
        rFields: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumParams(
        &self,
        phEnum: *mut *mut c_void,
        mb: u32,
        rParams: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMemberRefs(
        &self,
        phEnum: *mut *mut c_void,
        tkParent: u32,
        rMemberRefs: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMethodImpls(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rMethodBody: *mut u32,
        rMethodDecl: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumPermissionSets(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        dwActions: u32,
        rPermission: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindMember(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindMethod(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindField(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmb: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindMemberRef(
        &self,
        td: u32,
        szName: *const u16,
        pvSigBlob: *const u8,
        cbSigBlob: u32,
        pmr: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMethodProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szMethod: *mut u16,
        cchMethod: u32,
        pchMethod: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pulCodeRVA: *mut u32,
        pdwImplFlags: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMemberRefProps(
        &self,
        mr: u32,
        ptk: *mut u32,
        szMember: *mut u16,
        cchMember: u32,
        pchMember: *mut u32,
        ppvSigBlob: *mut *const u8,
        pbSig: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumProperties(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rProperties: *mut u32,
        cMax: u32,
        pcProperties: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumEvents(
        &self,
        phEnum: *mut *mut c_void,
        td: u32,
        rEvents: *mut u32,
        cMax: u32,
        pcEvents: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetEventProps(
        &self,
        ev: u32,
        pClass: *mut u32,
        szEvent: *mut u16,
        cchEvent: u32,
        pchEvent: *mut u32,
        pdwEventFlags: *mut u32,
        ptkEventType: *mut u32,
        pmdAddOn: *mut u32,
        pmdRemoveOn: *mut u32,
        pmdFire: *mut u32,
        rmdOtherMethod: *mut u32,
        cMax: u32,
        pcOtherMethod: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumMethodSemantics(
        &self,
        phEnum: *mut *mut c_void,
        mb: u32,
        rEventProp: *mut u32,
        cMax: u32,
        pcEventProp: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMethodSemantics(
        &self,
        mb: u32,
        tkEventProp: u32,
        pdwSemanticsFlags: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetClassLayout(
        &self,
        td: u32,
        pdwPackSize: *mut u32,
        rFieldOffset: *mut c_void,
        cMax: u32,
        pcFieldOffset: *mut u32,
        pulClassSize: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFieldMarshal(
        &self,
        tk: u32,
        ppvNativeType: *mut *const u8,
        pcbNativeType: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetRVA(&self, tk: u32, pulCodeRVA: *mut u32, pdwImplFlags: *mut u32) -> HRESULT;
    pub unsafe fn GetPermissionSetProps(
        &self,
        pm: u32,
        pdwAction: *mut u32,
        ppvPermission: *mut *const c_void,
        pcbPermission: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetSigFromToken(
        &self,
        mdSig: u32,
        ppvSig: *mut *const u8,
        pcbSig: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetModuleRefProps(
        &self,
        mur: u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumModuleRefs(
        &self,
        phEnum: *mut *mut c_void,
        rModuleRefs: *mut u32,
        cmax: u32,
        pcModuleRefs: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetTypeSpecFromToken(
        &self,
        typespec: u32,
        ppvSig: *mut *const u8,
        pcbSig: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetNameFromToken(&self, tk: u32, pszUtf8NamePtr: *mut *const u8) -> HRESULT;
    pub unsafe fn EnumUnresolvedMethods(
        &self,
        phEnum: *mut *mut c_void,
        rMethods: *mut u32,
        cMax: u32,
        pcTokens: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetUserString(
        &self,
        stk: u32,
        szString: *mut u16,
        cchString: u32,
        pchString: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetPinvokeMap(
        &self,
        tk: u32,
        pdwMappingFlags: *mut u32,
        szImportName: *mut u16,
        cchImportName: u32,
        pchImportName: *mut u32,
        pmrImportDLL: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumSignatures(
        &self,
        phEnum: *mut *mut c_void,
        rSignatures: *mut u32,
        cmax: u32,
        pcSignatures: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumTypeSpecs(
        &self,
        phEnum: *mut *mut c_void,
        rTypeSpecs: *mut u32,
        cmax: u32,
        pcTypeSpecs: *mut u32,
    ) -> HRESULT;
    pub unsafe fn EnumUserStrings(
        &self,
        phEnum: *mut *mut c_void,
        rStrings: *mut u32,
        cmax: u32,
        pcStrings: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetParentToken(&self, tk: u32, ptk: *mut u32) -> HRESULT;
    pub unsafe fn EnumCustomAttributes(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        tkType: u32,
        rCustomAttributes: *mut u32,
        cMax: u32,
        pcCustomAttributes: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetCustomAttributeProps(
        &self,
        cv: u32,
        ptkObj: *mut u32,
        ptkType: *mut u32,
        ppBlob: *mut *const c_void,
        pcbSize: *mut u32,
    ) -> HRESULT;
    pub unsafe fn FindTypeRef(
        &self,
        tkResolutionScope: u32,
        szName: *const u16,
        ptr: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetMemberProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szMember: *mut u16,
        cchMember: u32,
        pchMember: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pulCodeRVA: *mut u32,
        pdwImplFlags: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetFieldProps(
        &self,
        mb: u32,
        pClass: *mut u32,
        szField: *mut u16,
        cchField: u32,
        pchField: *mut u32,
        pdwAttr: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetPropertyProps(
        &self,
        prop: u32,
        pClass: *mut u32,
        szProperty: *mut u16,
        cchProperty: u32,
        pchProperty: *mut u32,
        pdwPropFlags: *mut u32,
        ppvSig: *mut *const u8,
        pbSig: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppDefaultValue: *mut *const c_void,
        pcchDefaultValue: *mut u32,
        pmdSetter: *mut u32,
        pmdGetter: *mut u32,
        rmdOtherMethod: *mut u32,
        cMax: u32,
        pcOtherMethod: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetParamProps(
        &self,
        tk: u32,
        pmd: *mut u32,
        pulSequence: *mut u32,
        szName: *mut u16,
        cchName: u32,
        pchName: *mut u32,
        pdwAttr: *mut u32,
        pdwCPlusTypeFlag: *mut u32,
        ppValue: *mut *const c_void,
        pcchValue: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetCustomAttributeByName(
        &self,
        tkObj: u32,
        szName: *const u16,
        ppData: *mut *const c_void,
        pcbData: *mut u32,
    ) -> HRESULT;
    pub unsafe fn IsValidToken(&self, tk: u32) -> i32;
    pub unsafe fn GetNestedClassProps(
        &self,
        tdNestedClass: u32,
        ptdEnclosingClass: *mut u32,
    ) -> HRESULT;
    pub unsafe fn GetNativeCallConvFromSig(
        &self,
        pvSig: *const c_void,
        cbSig: u32,
        pCallConv: *mut u32,
    ) -> HRESULT;
    pub unsafe fn IsGlobal(&self, pd: u32, pbGlobal: *mut i32) -> HRESULT;

    // IMetaDataImport2 specific methods
    /// Enumerate generic parameters.
    pub unsafe fn EnumGenericParams(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        rGenericParams: *mut u32,
        cMax: u32,
        pcGenericParams: *mut u32,
    ) -> HRESULT;

    /// Get generic parameter properties.
    pub unsafe fn GetGenericParamProps(
        &self,
        gp: u32,
        pulParamSeq: *mut u32,
        pdwParamFlags: *mut u32,
        ptOwner: *mut u32,
        reserved: *mut u32,
        wzname: *mut u16,
        cchName: u32,
        pchName: *mut u32,
    ) -> HRESULT;

    /// Get method spec properties.
    pub unsafe fn GetMethodSpecProps(
        &self,
        mi: u32,
        tkParent: *mut u32,
        ppvSigBlob: *mut *const u8,
        pcbSigBlob: *mut u32,
    ) -> HRESULT;

    /// Enumerate method specs.
    pub unsafe fn EnumMethodSpecs(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        rMethodSpecs: *mut u32,
        cMax: u32,
        pcMethodSpecs: *mut u32,
    ) -> HRESULT;

    /// Enumerate generic parameter constraints.
    pub unsafe fn EnumGenericParamConstraints(
        &self,
        phEnum: *mut *mut c_void,
        tk: u32,
        rGenericParamConstraints: *mut u32,
        cMax: u32,
        pcGenericParamConstraints: *mut u32,
    ) -> HRESULT;

    /// Get generic parameter constraint properties.
    pub unsafe fn GetGenericParamConstraintProps(
        &self,
        gpc: u32,
        ptGenericParam: *mut u32,
        ptkConstraintType: *mut u32,
    ) -> HRESULT;

    /// Get the PE kind.
    pub unsafe fn GetPEKind(&self, pdwPEKind: *mut u32, pdwMachine: *mut u32) -> HRESULT;

    /// Get the version string.
    pub unsafe fn GetVersionString(
        &self,
        pwzBuf: *mut u16,
        ccBufSize: u32,
        pccBufSize: *mut u32,
    ) -> HRESULT;
}
