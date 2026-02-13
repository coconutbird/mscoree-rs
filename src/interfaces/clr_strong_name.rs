//! ICLRStrongName interfaces for strong name signing.
//!
//! These interfaces provide strong name key management and signing.

use std::ffi::c_void;
use windows::core::{HRESULT, IUnknown, IUnknown_Vtbl, interface, PCWSTR};

/// ICLRStrongName - Strong name signing and verification.
#[interface("9FD93CCF-3280-4391-B3A9-96E1CDE77C8D")]
pub unsafe trait ICLRStrongName: IUnknown {
    /// Get blob from key file.
    pub unsafe fn GetHashFromAssemblyFile(
        &self,
        pszFilePath: PCWSTR,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Get blob from key file (wide).
    pub unsafe fn GetHashFromAssemblyFileW(
        &self,
        pwzFilePath: PCWSTR,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Get hash from blob.
    pub unsafe fn GetHashFromBlob(
        &self,
        pbBlob: *const u8,
        cchBlob: u32,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Get hash from file.
    pub unsafe fn GetHashFromFile(
        &self,
        pszFilePath: PCWSTR,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Get hash from file (wide).
    pub unsafe fn GetHashFromFileW(
        &self,
        pwzFilePath: PCWSTR,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Get hash from HANDLE.
    pub unsafe fn GetHashFromHandle(
        &self,
        hFile: *mut c_void,
        piHashAlg: *mut u32,
        pbHash: *mut u8,
        cchHash: u32,
        pchHash: *mut u32,
    ) -> HRESULT;

    /// Free key blob buffer.
    pub unsafe fn StrongNameFreeBuffer(&self, pbMemory: *mut u8) -> HRESULT;

    /// Generate a new key pair.
    pub unsafe fn StrongNameKeyGen(
        &self,
        pwzKeyContainer: PCWSTR,
        dwFlags: u32,
        ppbKeyBlob: *mut *mut u8,
        pcbKeyBlob: *mut u32,
    ) -> HRESULT;

    /// Generate a new key pair with size.
    pub unsafe fn StrongNameKeyGenEx(
        &self,
        pwzKeyContainer: PCWSTR,
        dwFlags: u32,
        dwKeySize: u32,
        ppbKeyBlob: *mut *mut u8,
        pcbKeyBlob: *mut u32,
    ) -> HRESULT;

    /// Delete a key container.
    pub unsafe fn StrongNameKeyDelete(&self, pwzKeyContainer: PCWSTR) -> HRESULT;

    /// Install a key pair into a container.
    pub unsafe fn StrongNameKeyInstall(
        &self,
        pwzKeyContainer: PCWSTR,
        pbKeyBlob: *const u8,
        cbKeyBlob: u32,
    ) -> HRESULT;

    /// Get public key from key pair.
    pub unsafe fn StrongNameGetPublicKey(
        &self,
        pwzKeyContainer: PCWSTR,
        pbKeyBlob: *const u8,
        cbKeyBlob: u32,
        ppbPublicKeyBlob: *mut *mut u8,
        pcbPublicKeyBlob: *mut u32,
    ) -> HRESULT;

    /// Get public key token.
    pub unsafe fn StrongNameTokenFromPublicKey(
        &self,
        pbPublicKeyBlob: *const u8,
        cbPublicKeyBlob: u32,
        ppbStrongNameToken: *mut *mut u8,
        pcbStrongNameToken: *mut u32,
    ) -> HRESULT;

    /// Get public key blob size.
    pub unsafe fn StrongNameGetBlobFromImage(
        &self,
        pbBase: *const u8,
        dwLength: u32,
        pbBlob: *mut u8,
        pcbBlob: *mut u32,
    ) -> HRESULT;

    /// Get public key blob.
    pub unsafe fn StrongNameGetBlob(
        &self,
        pwzFilePath: PCWSTR,
        pbBlob: *mut u8,
        pcbBlob: *mut u32,
    ) -> HRESULT;

    /// Sign assembly.
    pub unsafe fn StrongNameSignatureGeneration(
        &self,
        pwzFilePath: PCWSTR,
        pwzKeyContainer: PCWSTR,
        pbKeyBlob: *const u8,
        cbKeyBlob: u32,
        ppbSignatureBlob: *mut *mut u8,
        pcbSignatureBlob: *mut u32,
    ) -> HRESULT;

    /// Sign assembly with extended flags.
    pub unsafe fn StrongNameSignatureGenerationEx(
        &self,
        wszFilePath: PCWSTR,
        wszKeyContainer: PCWSTR,
        pbKeyBlob: *const u8,
        cbKeyBlob: u32,
        ppbSignatureBlob: *mut *mut u8,
        pcbSignatureBlob: *mut u32,
        dwFlags: u32,
    ) -> HRESULT;

    /// Get signature size.
    pub unsafe fn StrongNameSignatureSize(
        &self,
        pbPublicKeyBlob: *const u8,
        cbPublicKeyBlob: u32,
        pcbSize: *mut u32,
    ) -> HRESULT;

    /// Verify strong name signature.
    pub unsafe fn StrongNameSignatureVerification(
        &self,
        pwzFilePath: PCWSTR,
        dwInFlags: u32,
        pdwOutFlags: *mut u32,
    ) -> HRESULT;

    /// Verify strong name signature with extended flags.
    pub unsafe fn StrongNameSignatureVerificationEx(
        &self,
        pwzFilePath: PCWSTR,
        fForceVerification: i32,
        pfWasVerified: *mut i32,
    ) -> HRESULT;

    /// Verify strong name signature from image.
    pub unsafe fn StrongNameSignatureVerificationFromImage(
        &self,
        pbBase: *const u8,
        dwLength: u32,
        dwInFlags: u32,
        pdwOutFlags: *mut u32,
    ) -> HRESULT;

    /// Get token from assembly.
    pub unsafe fn StrongNameTokenFromAssembly(
        &self,
        pwzFilePath: PCWSTR,
        ppbStrongNameToken: *mut *mut u8,
        pcbStrongNameToken: *mut u32,
    ) -> HRESULT;

    /// Get token and public key from assembly.
    pub unsafe fn StrongNameTokenFromAssemblyEx(
        &self,
        pwzFilePath: PCWSTR,
        ppbStrongNameToken: *mut *mut u8,
        pcbStrongNameToken: *mut u32,
        ppbPublicKeyBlob: *mut *mut u8,
        pcbPublicKeyBlob: *mut u32,
    ) -> HRESULT;

    /// Compare assemblies.
    pub unsafe fn StrongNameCompareAssemblies(
        &self,
        pwzAssembly1: PCWSTR,
        pwzAssembly2: PCWSTR,
        pdwResult: *mut u32,
    ) -> HRESULT;

    /// Hash assembly file.
    pub unsafe fn StrongNameHashSize(
        &self,
        ulHashAlg: u32,
        pcbSize: *mut u32,
    ) -> HRESULT;
}

