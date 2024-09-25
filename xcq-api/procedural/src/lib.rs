/// Declare the calls used in XCQ program
/// ```ignore
/// #[xcq::program]
/// mod query_fungibles {
///     #[xcq::call_def(extension_id = 123456, extern_types = [AssetId, AccountId, Balance])]
///     fn balance(asset: AssetId, who: AccountId) -> Balance;
///
///     #[xcq::entrypoint]
///     fn sum_balance(calls: Vec<Call>) -> u64 {
///         let mut sum = 0;
///         for call in calls {
///            // calculation requires a known balance type, we can use assert-type here
///             sum += call.call();
///         }
///         sum
///     }
/// }
/// ```
///
mod program;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn program(attr: TokenStream, item: TokenStream) -> TokenStream {
    program::program(attr, item)
}
