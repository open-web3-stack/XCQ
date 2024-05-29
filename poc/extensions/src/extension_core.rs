use super::{Extension, ExtensionTypeId};
pub trait ExtensionCore: Extension {
    // ExtensionId should be generated by the macro
    // It should normalize the order of methods and parameter names
    const TYPE_ID: ExtensionTypeId = 0u64;
    // TODO: Actual args and return values are complex types
    // and we adapt them to polkavm ABI in `impl XcqExecutorContext for HostFunctions`
    fn some_host_function() -> u32;
    fn another_host_function() -> u32;
}

#[derive(Clone)]
struct ExtensionCoreImpl;

impl Extension for ExtensionCoreImpl {
    fn methods(&self) -> Vec<String> {
        vec!["some_host_function".to_string(), "another_host_function".to_string()]
    }
}

impl ExtensionCore for ExtensionCoreImpl {
    fn some_host_function() -> u32 {
        100
    }
    fn another_host_function() -> u32 {
        42
    }
}
