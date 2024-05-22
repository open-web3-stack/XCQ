#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

pub type XcqResponse = Vec<u8>;

pub enum XcqError {
    Custom(&'static str),
}

pub type XcqResult = Result<XcqResponse, XcqError>;
