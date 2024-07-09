use crate::{boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};
use crate::{EnumType, Field, PrimitiveType, StructType, Variant};
use crate::{XcqType, XcqTypeInfo};
use fortuples::fortuples;

macro_rules! impl_from_xcqtype_variant {
    ($($from:ty => $variant: ident,)*) => {
        $(
            impl From<$from> for XcqType {
                fn from(x: $from) -> Self {
                    Self::$variant(x)
                }
            }
        )*
    };
}

impl_from_xcqtype_variant! {
    StructType => Struct,
    EnumType => Enum,
    PrimitiveType => Primitive,
}

// Implement `XcqTypeInfo` for primitive types.
macro_rules! impl_metadata_for_primitives {
    ( $($t:ty=> $ident_kind:expr ),*) => {
        $(
            impl XcqTypeInfo for $t {
                type Identity = Self;
                fn type_info() -> XcqType {
                    $ident_kind.into()
                }
            }
        )*
    }
}
impl_metadata_for_primitives! {
    bool => PrimitiveType::Bool,
    u8 => PrimitiveType::U8,
    u16 => PrimitiveType::U16,
    u32 => PrimitiveType::U32,
    u64 => PrimitiveType::U64,
    u128 => PrimitiveType::U128,
    i8 => PrimitiveType::I8,
    i16 => PrimitiveType::I16,
    i32 => PrimitiveType::I32,
    i64 => PrimitiveType::I64,
    i128 => PrimitiveType::I128,
    [u8;32] => PrimitiveType::H256
}

fortuples! {
    impl XcqTypeInfo for #Tuple where #(#Member: XcqTypeInfo+'static),* {
        type Identity = Self;
        fn type_info() -> XcqType {
            XcqType::Tuple(vec![ #(#Member::type_info()),* ])
        }
    }
}

impl<T> XcqTypeInfo for Option<T>
where
    T: XcqTypeInfo + 'static,
{
    type Identity = Self;
    fn type_info() -> XcqType {
        EnumType {
            ident: b"Option".to_vec(),
            variants: vec![
                Variant {
                    ident: b"None".to_vec(),
                    fields: vec![],
                },
                Variant {
                    ident: b"Some".to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: T::type_info(),
                    }],
                },
            ],
        }
        .into()
    }
}

impl<T, E> XcqTypeInfo for Result<T, E>
where
    T: XcqTypeInfo + 'static,
    E: XcqTypeInfo + 'static,
{
    type Identity = Self;
    fn type_info() -> XcqType {
        EnumType {
            ident: b"Result".to_vec(),
            variants: vec![
                Variant {
                    ident: b"Ok".to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: T::type_info(),
                    }],
                },
                Variant {
                    ident: b"Err".to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: E::type_info(),
                    }],
                },
            ],
        }
        .into()
    }
}

macro_rules! impl_metadata_for_smart_pointers {
    ($($type:ty),*) => {
        $(
            impl<T> XcqTypeInfo for $type
            where
                T: XcqTypeInfo + ?Sized + 'static,
            {
                type Identity = T;
                fn type_info() -> XcqType {
                    Self::Identity::type_info()
                }
            }
        )*
    };
}

impl_metadata_for_smart_pointers! {
    Box<T>,
    Arc<T>,
    Rc<T>,
    &T,
    &mut T
}

impl<T> XcqTypeInfo for [T]
where
    T: XcqTypeInfo + 'static,
{
    type Identity = Self;
    fn type_info() -> XcqType {
        XcqType::Sequence(Box::new(T::type_info()))
    }
}

impl<T> XcqTypeInfo for Vec<T>
where
    T: XcqTypeInfo + 'static,
{
    type Identity = [T];
    fn type_info() -> XcqType {
        Self::Identity::type_info()
    }
}

impl XcqTypeInfo for str {
    type Identity = str;
    fn type_info() -> XcqType {
        XcqType::Sequence(Box::new(PrimitiveType::U8.into()))
    }
}

impl XcqTypeInfo for String {
    type Identity = str;
    fn type_info() -> XcqType {
        Self::Identity::type_info()
    }
}
// No impl for PhantomData, codec::Compact, Range, RangeInclusive, str, BTreeMap, BTreeSet, BinaryHeap, VecDeque yet.
// No support for self-referential types yet.
