use xcq_types::{XcqType, XcqTypeInfo};
pub fn assert_type_fn<T: XcqTypeInfo + ?Sized>(expect: XcqType) {
    assert_eq!(T::type_info(), expect);
}

#[macro_export]
macro_rules! assert_type {
    ($ty:ty, $expected: expr) => {{
        $crate::macros::assert_type_fn::<$ty>($expected);
    }};
}
