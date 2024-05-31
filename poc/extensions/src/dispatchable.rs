pub trait Dispatchable {
    type MethodName;
    type MethodIndex;
    fn query_method(method_name: Self::MethodName) -> Self::MethodIndex;
    fn dispatch(self) -> Vec<u8>;
}

pub type ExtensionTypeId = u64;
