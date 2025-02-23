#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type PvqResponse = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum PvqError {
    Custom(String),
}

pub type PvqResult = Result<PvqResponse, PvqError>;
