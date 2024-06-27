#![cfg_attr(not(feature = "std"), no_std)]

mod ty;
pub use ty::*;

pub trait TypeInfo {
    fn type_info() -> Type;
}
