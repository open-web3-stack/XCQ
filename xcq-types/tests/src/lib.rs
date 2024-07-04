use xcq_types::XcqType;
#[derive(XcqTypeInfo)]
struct Person {
    name: String,
    age_in_years: u8,
}
#[test]
fn struct_types() {
    assert_type!(
        Person,
        XcqType::Struct(StructType {
            ident: "Person".as_bytes().to_vec(),
            fields: vec![
                Field {
                    ident: "name".as_bytes().to_vec(),
                    ty: XcqType::Sequence(Box::new(XcqType::Primitive(Unsigned))),
                },
                Field {
                    ident: "age_in_years".as_bytes().to_vec(),
                    ty: PrimitiveType::U8.into(),
                },
            ],
        })
    );
}
