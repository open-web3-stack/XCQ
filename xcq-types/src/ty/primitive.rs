pub enum PrimitiveType {
    /// Unsigned Integer up to 128 bits
    Unsigned,
    /// Signed Integer up to 128 bits
    Signed,
    U256,
    I256,
    Bool,
    /// Vec<u8>
    Bytes,
    /// [u8; 32]
    H256,
}
