#![cfg_attr(not(feature = "std"), no_std)]

use poc_executor::{XcqExecutor, XcqExecutorContext};
pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

mod macros;

type ExtensionTypeId = u32;
type Error = String;

// Runtime Side
// General trait for all extensions
pub trait Extension {
    fn type_id(&self) -> ExtensionTypeId;
    fn methods(&self) -> Vec<String>;
}

// Example extension trait
// Implemented by Runtime
pub trait ExtensionCore {
    type HostFunctions;
    // TODO: should be generated automatically by macro
    const EXTENSION_TYPE_ID: ExtensionTypeId;
    fn core_fn(&self) -> Result<(), Error>;
}

type Method = String;
// Which should be implemented by the runtime
pub trait ExtensionsExecutor {
    type SafeGuard: PermController;
    fn support_extension_types() -> Result<Vec<ExtensionTypeId>, Error>;
    fn discover_methods<E: Extension>(extension: &E) -> Result<Vec<Method>, Error>;
    fn execute_method<E: Extension>(&self, extension: E, method: Method, input: Vec<u8>) -> XcqResult;
}
pub trait PermController {
    fn check<E: Extension>(extension: &E, method: &Method) -> Result<(), Error>;
}

struct SimplePermController;
impl PermController for SimplePermController {
    fn check<E: Extension>(extension: &E, method: &Method) -> Result<(), Error> {
        unimplemented!()
    }
}

// Mock implementation
struct ExtensionApiImpl;
impl ExtensionsExecutor for ExtensionApiImpl {
    type SafeGuard = SimplePermController;
    fn support_extension_types() -> Result<Vec<ExtensionTypeId>, Error> {
        unimplemented!()
    }
    // TODO: Actually, extension is opaque to the runtime,
    // we need to use polkavm discover symbols to get the methods
    fn discover_methods<E: Extension>(extension: &E) -> Result<Vec<Method>, Error> {
        Ok(extension.methods())
    }
    fn execute_method<E: Extension>(&self, extension: E, method: Method, input: Vec<u8>) -> XcqResult {
        // Check if the extension is supported
        let extension_ids = Self::support_extension_types()?;
        if !extension_ids.contains(&extension.type_id()) {
            return Err("Extension not supported".to_string());
        }
        // Check if the method is supported
        let methods = Self::discover_methods(&extension)?;
        if !methods.contains(&method) {
            return Err("Method not supported".to_string());
        }
        // Check if the method pass the safe guard
        Self::SafeGuard::check(&extension, &method)?;
        // TODO: Execute the method
        Ok(vec![])
    }
}
