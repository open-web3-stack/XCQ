use parity_scale_codec::Encode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Debug)]
pub enum PrimitiveType {
    Bool,
    Char,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    // TODO: representation
    U256,
    // TODO: representation
    I256,
    /// [u8; 32]
    H256,
    // TODO: more fixed-size arrays represented as primitives
}
