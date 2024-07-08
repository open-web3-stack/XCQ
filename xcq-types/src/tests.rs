use crate::{EnumType, Field, PrimitiveType, StructType, Variant, XcqType, XcqTypeInfo};

fn assert_type_fn<T: XcqTypeInfo + ?Sized>(expect: XcqType) {
    assert_eq!(T::type_info(), expect);
}

macro_rules! assert_type {
    ($ty:ty, $expected: expr) => {{
        assert_type_fn::<$ty>($expected);
    }};
}

#[test]
fn primitives() {
    assert_type!(bool, XcqType::Primitive(PrimitiveType::Bool));
    assert_type!(&str, XcqType::Sequence(Box::new(PrimitiveType::U8.into())));
    assert_type!(i8, XcqType::Primitive(PrimitiveType::I8));
    assert_type!([bool], XcqType::Sequence(Box::new(PrimitiveType::Bool.into())));
}

#[test]
fn prelude_items() {
    assert_type!(String, XcqType::Sequence(Box::new(PrimitiveType::U8.into())));

    assert_type!(
        Option<u128>,
        XcqType::Enum(EnumType {
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
                        ty: PrimitiveType::U128.into(),
                    }],
                },
            ],
        })
    );

    assert_type!(
        Result<bool, String>,
        XcqType::Enum(EnumType {
            ident: b"Result".to_vec(),
            variants: vec![
                Variant {
                    ident: b"Ok".to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: PrimitiveType::Bool.into(),
                    }],
                },
                Variant {
                    ident: b"Err".to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: XcqType::Sequence(Box::new(PrimitiveType::U8.into())),
                    }],
                },
            ],
        })
    );
}

#[test]
fn tuple_primitives() {
    assert_type!((), XcqType::Tuple(vec![]));
    assert_type!((bool,), XcqType::Tuple(vec![PrimitiveType::Bool.into()]));
    assert_type!(
        (bool, i8),
        XcqType::Tuple(vec![PrimitiveType::Bool.into(), PrimitiveType::I8.into()])
    );
    assert_type!(
        ((i8, i16), (u32, u64)),
        XcqType::Tuple(vec![
            XcqType::Tuple(vec![PrimitiveType::I8.into(), PrimitiveType::I16.into()]),
            XcqType::Tuple(vec![PrimitiveType::U32.into(), PrimitiveType::U64.into()]),
        ])
    );
}

#[derive(XcqTypeInfo)]
struct Person {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    age_in_years: u8,
}
#[test]
fn struct_types() {
    assert_type!(
        Person,
        XcqType::Struct(StructType {
            ident: b"Person".to_vec(),
            fields: vec![
                Field {
                    ident: b"name".to_vec(),
                    ty: XcqType::Sequence(Box::new(XcqType::Primitive(PrimitiveType::U8.into()))),
                },
                Field {
                    ident: b"age_in_years".to_vec(),
                    ty: PrimitiveType::U8.into(),
                },
            ],
        })
    );
}
