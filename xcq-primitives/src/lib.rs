#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

pub type XcqResponse = Vec<u8>;
pub type XcqError = ();
pub type XcqResult = Result<XcqResponse, XcqError>;
