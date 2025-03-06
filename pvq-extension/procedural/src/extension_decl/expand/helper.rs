use crate::extension_decl::parse::extension::ExtensionMethod;
use crate::utils::generate_crate_access;
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

// Calculate hash for extension ID
pub fn calculate_hash(trait_ident: &syn::Ident, methods: &[ExtensionMethod]) -> u64 {
    let mut hasher = XxHash64::default();
    // reduce the chance of hash collision
    "pvq-ext$".hash(&mut hasher);
    trait_ident.hash(&mut hasher);
    for method in methods {
        // reduce the chance of hash collision
        "@".hash(&mut hasher);
        method.name.hash(&mut hasher);
    }
    hasher.finish()
}

// Add super trait ExtensionId and ExtensionMetadata to the trait's where clause
pub fn add_super_trait(item_trait: &mut syn::ItemTrait) -> syn::Result<()> {
    let pvq_extension = generate_crate_access("pvq-extension")?;
    item_trait
        .supertraits
        .push(syn::parse_quote!(#pvq_extension::ExtensionImplMetadata));
    Ok(())
}
