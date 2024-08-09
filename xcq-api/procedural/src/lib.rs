/// Declare the calls used in XCQ program
/// #[xcq::program]
/// mod query_fungibles {
///     #[xcq::call(extern_types = [AssetId, AccountId, Balance]])]]
///     fn balance(asset: AssetId, who: AccountId) -> Balance;
///
///     #[xcq(entrypoint)]
///     fn sum_balance(calls: Vec<Call>) -> u64 {
///         let mut sum = 0;
///         for call in calls {
///             sum += call.call();
///         }
///         sum
///     }
/// }
///
mod program;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn program(attr: TokenStream, item: TokenStream) -> TokenStream {
    program::program(attr, item)
}
