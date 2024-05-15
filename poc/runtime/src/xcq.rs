use frame::deps::sp_api::decl_runtime_apis;
use frame::prelude::*;
#[allow(unused_imports)]
use scale_info::prelude::{format, string::String};

pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

decl_runtime_apis! {
    pub trait XcqApi {
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
    }
}

struct HostFunctions;

impl poc_executor::XcqExecutorContext for HostFunctions {
    fn register_host_functions<T>(&mut self, linker: &mut poc_executor::Linker<T>) {
        linker.func_wrap("host_call", || -> u32 { 100 }).unwrap();
    }
}

pub fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult {
    let mut executor = poc_executor::XcqExecutor::new(Default::default(), HostFunctions);
    executor.execute(&query, &input).map_err(|e| format!("{:?}", e))
}
