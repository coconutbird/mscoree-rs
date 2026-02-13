//! # mscoree
//!
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//!
//! Rust bindings for mscoree COM interfaces for interacting with CLR and COR runtimes.
//!
//! This crate provides safe Rust wrappers around the COM interfaces defined in mscoree.h
//! and metahost.h for hosting the .NET Common Language Runtime (CLR).
//!
//! ## Key Interfaces
//!
//! - [`ICLRMetaHost`] - Entry point for CLR hosting (.NET 4.0+)
//! - [`ICLRRuntimeInfo`] - Information about a specific CLR version
//! - [`ICLRRuntimeHost`] - Runtime hosting interface (.NET 2.0+)
//! - [`ICorRuntimeHost`] - Legacy runtime hosting interface (.NET 1.x)
//!
//! ## Example
//!
//! ```no_run
//! use mscoree::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost};
//! use windows::core::PCWSTR;
//!
//! unsafe {
//!     // Get the meta host
//!     let meta_host: ICLRMetaHost = CLRCreateInstance(&CLSID_CLRMetaHost)?;
//!     
//!     // Get runtime info for .NET 4.0
//!     let version = windows::core::w!("v4.0.30319");
//!     let runtime_info = meta_host.GetRuntime(version)?;
//! }
//! # Ok::<(), windows::core::Error>(())
//! ```

mod functions;
mod guids;
mod interfaces;

pub use functions::*;
pub use guids::*;
pub use interfaces::*;
