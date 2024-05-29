use super::{Extension, ExtensionTypeId};
pub trait ExtensionFungibles: Extension {
    // ExtensionId should be generated by the macro
    // It should normalize the order of methods and parameter names
    const TYPE_ID: ExtensionTypeId = 1u64;
    // TODO: Actual args and return values are complex types
    // and we adapt them to polkavm ABI in `impl XcqExecutorContext for HostFunctions`
    fn transfer(from: u32, to: u32, amount: u64) -> u64;
    fn balance(account: u32) -> u64;
}

#[derive(Clone)]
struct ExtensionFungiblesImpl;

impl Extension for ExtensionFungiblesImpl {
    fn methods(&self) -> Vec<String> {
        vec!["transfer".to_string(), "balance".to_string()]
    }
}

impl ExtensionFungibles for ExtensionFungiblesImpl {
    fn transfer(_from: u32, _to: u32, _amount: u64) -> u64 {
        unimplemented!()
    }
    fn balance(_account: u32) -> u64 {
        unimplemented!()
    }
}
