#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

mod ty;
pub use ty::*;
mod impls;
mod prelude;
pub use prelude::*;
mod meta_type;
pub use meta_type::MetaType;
#[cfg(test)]
mod tests;

pub use xcq_types_derive::XcqTypeInfo;

/// Implementors return the meta type information.
pub trait XcqTypeInfo {
    /// This is used to uniquely identify the type via [`core::any::TypeId::of`]
    /// In most case it is Self, but for reference types it is the type of the reference.
    type Identity: ?Sized + 'static;
    fn type_info() -> XcqType;
}
/// helper trait for combining `XcqTypeInfo` and `'static
pub trait XcqStaticTypeInfo: XcqTypeInfo + 'static {}

impl<T> XcqStaticTypeInfo for T where T: XcqTypeInfo + 'static {}

/// Returns the runtime bridge to the types compile-time type information.
pub fn meta_type<T>() -> MetaType
where
    T: ?Sized + XcqTypeInfo + 'static,
{
    MetaType::new::<T>()
}
