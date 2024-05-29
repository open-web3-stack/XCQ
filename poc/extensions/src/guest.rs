use super::ExtensionTypeId;
pub trait Guest {
    fn type_id(&self) -> ExtensionTypeId;
    fn program(&self) -> &[u8];
}

struct GuestImpl {
    extension_type: ExtensionTypeId,
    program: Vec<u8>,
}

impl Guest for GuestImpl {
    fn type_id(&self) -> ExtensionTypeId {
        self.extension_type
    }
    fn program(&self) -> &[u8] {
        &self.program
    }
}

type Method = String;

pub trait Input {
    fn method(&self) -> Method;
    fn args(&self) -> &[u8];
}

struct InputImpl {
    method: Method,
    args: Vec<u8>,
}

impl Input for InputImpl {
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn args(&self) -> &[u8] {
        &self.args
    }
}
