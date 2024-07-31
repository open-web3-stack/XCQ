#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type XcqResponse = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum XcqError {
    Custom(String),
}

pub type XcqResult = Result<XcqResponse, XcqError>;

pub mod metadata;
pub mod metadata_ir;

pub mod deps {
    pub use parity_scale_codec;
    pub use xcq_types;
}
