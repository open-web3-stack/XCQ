use crate::assert_type;
use xcq_types::{EnumType, Field, Variant};
use xcq_types::{PrimitiveType, XcqType, XcqTypeInfo};

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
            ident: "Option".as_bytes().to_vec(),
            variants: vec![
                Variant {
                    ident: "None".as_bytes().to_vec(),
                    fields: vec![],
                },
                Variant {
                    ident: "Some".as_bytes().to_vec(),
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
            ident: "Result".as_bytes().to_vec(),
            variants: vec![
                Variant {
                    ident: "Ok".as_bytes().to_vec(),
                    fields: vec![Field {
                        ident: vec![],
                        ty: PrimitiveType::Bool.into(),
                    }],
                },
                Variant {
                    ident: "Err".as_bytes().to_vec(),
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
