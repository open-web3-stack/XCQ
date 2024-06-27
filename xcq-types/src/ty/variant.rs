use super::Fields;

pub struct Variant {
    ident: Vec<u8>,
    fields: Fields,
}

pub struct EnumType {
    ident: Vec<u8>,
    variants: Vec<Variant>,
}
