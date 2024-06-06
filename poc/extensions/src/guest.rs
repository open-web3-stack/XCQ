pub trait Guest {
    fn program(&self) -> &[u8];
}

pub type Method = String;

pub trait Input {
    fn method(&self) -> Method;
    fn args(&self) -> &[u8];
}
