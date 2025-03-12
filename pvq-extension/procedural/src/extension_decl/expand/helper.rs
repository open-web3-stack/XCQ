use crate::extension_decl::parse::extension::ExtensionFunction;
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

// Calculate hash for extension ID
pub fn calculate_hash(trait_ident: &syn::Ident, functions: &[ExtensionFunction]) -> u64 {
    let mut hasher = XxHash64::default();
    // reduce the chance of hash collision
    "pvq-ext$".hash(&mut hasher);
    trait_ident.hash(&mut hasher);
    for function in functions {
        // reduce the chance of hash collision
        "@".hash(&mut hasher);
        function.name.hash(&mut hasher);
    }
    hasher.finish()
}
