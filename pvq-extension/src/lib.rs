#![cfg_attr(not(feature = "std"), no_std)]
//! # PVQ Extension System
//!
//! This crate provides an extension system for PVQ (PolkaVM Query).
//! It allows defining and implementing extensions that can be called from PVQ queries.
//!
//! ## Overview
//!
//! The extension system consists of:
//!
//! - **Extension Definitions**: Traits that define the API of an extension
//! - **Extension Implementations**: Implementations of extension traits
//! - **Extension Executor**: A runtime for executing queries
//! - **Permission Control**: Access control mechanisms to control access to extensions
//!
//! ## Usage
//!
//! Extensions are defined using the `#[extension_decl]` macro and implemented using the
//! `#[extensions_impl]` macro. See the examples directory for usage examples.

// Re-exports
pub use pvq_extension_procedural::{extension_decl, extensions_impl};

// Module declarations
mod calldata;
mod context;
mod error;
mod executor;
mod macros;
pub mod metadata;
mod perm_controller;

// Public exports
pub use calldata::{CallData, DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};
pub use context::Context;
pub use error::ExtensionError;
pub use executor::ExtensionsExecutor;
pub use macros::CallDataTuple;
pub use metadata::{ExtensionImplMetadata, ExtensionMetadata};
pub use perm_controller::{InvokeSource, PermissionController};
