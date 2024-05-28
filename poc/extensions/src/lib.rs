#![cfg_attr(not(feature = "std"), no_std)]

use poc_executor::{XcqExecutor, XcqExecutorContext};
pub type XcqResponse = Vec<u8>;
pub type XcqError = String;
pub type XcqResult = Result<XcqResponse, XcqError>;

mod macros;

type ExtensionTypeId = u32;
type Error = String;

// Runtime Side
// General trait for all host extension interfaces
pub trait HostInterface {
    fn type_id(&self) -> ExtensionTypeId;
    fn methods(&self) -> Vec<String>;
}

// Example extension trait
// Implemented by Runtime
pub trait SomeHostInterface {
    type HostFunctions: XcqExecutorContext;
    // TODO: should be generated automatically by macro
    const EXTENSION_TYPE_ID: ExtensionTypeId;
    fn some_host_function() -> XcqResult;
    fn another_host_function() -> XcqResult;
}

// Guest Side
pub trait Guest {
    fn type_id(&self) -> ExtensionTypeId;
    fn methods(&self) -> Vec<String>;
}

struct GuestImpl {
    program: Vec<u8>,
}

impl Guest for GuestImpl {
    fn type_id(&self) -> ExtensionTypeId {
        unimplemented!()
    }
    fn methods(&self) -> Vec<String> {
        unimplemented!()
    }
}

type Method = String;
// Which should be implemented by the runtime
pub trait ExtensionsExecutor {
    type SafeGuard: PermController;
    fn register_host_interface<H: HostInterface>(&mut self, host_interface: H) -> Result<(), Error>;
    fn support_extension_types() -> Result<Vec<ExtensionTypeId>, Error>;
    // extension type is opaque to the runtime
    // or we parse it before
    fn discover_methods<G: Guest>(guest: G) -> Result<Vec<Method>, Error>;
    fn execute_method<G: Guest>(&self, guest: G, method: Method, input: Vec<u8>) -> XcqResult;
}
pub trait PermController {
    fn check<G: Guest>(extension: &E, method: &Method) -> Result<(), Error>;
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
    fn register_host_interface<H: HostInterface>(&mut self, host_interface: H) -> Result<(), Error> {
        unimplemented!()
    }
    fn discover_methods<G: Guest>(guest_impl: G) -> Result<Vec<Method>, Error> {
        // TODO: extension is opaque to the runtime,
        // we need to use polkavm discover symbols to get the methods
        unimplemented!()
    }
    fn execute_method<G: Guest>(&self, guest: G, method: Method, input: Vec<u8>) -> XcqResult {
        // Check if the extension is supported
        let extension_ids = Self::support_extension_types()?;
        if !extension_ids.contains(&guest.type_id()) {
            return Err("Extension not supported".to_string());
        }
        // Check if the method is supported
        let methods = Self::discover_methods(&guest)?;
        if !methods.contains(&method) {
            return Err("Method not supported".to_string());
        }
        // Check if the method pass the safe guard
        Self::SafeGuard::check(&guest, &method)?;
        // TODO: Execute the method
        Ok(vec![])
    }
}
