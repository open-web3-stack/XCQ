use std::collections::HashSet;

use crate::utils::{extract_impl_trait, generate_crate_access, generate_mod_name_for_trait, RequireQualifiedTraitPath};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_quote, punctuated::Punctuated, visit_mut::VisitMut, GenericParam, Generics, ItemImpl, ItemTrait, Result,
    Token,
};

/// Generate the runtime metadata of the provided extension trait.
///
/// The metadata is exposed as a generic function on the hidden module
/// of the trait generated by the `decl_extensions`.
pub fn generate_decl_metadata(decl: &ItemTrait, has_config: bool) -> Result<TokenStream2> {
    let xcq_primitives = generate_crate_access("xcq-primitives")?;
    let umbrella = quote! { #xcq_primitives::umbrella };
    let mut methods = Vec::new();

    // Convert `<Self::Config as Config>::Associated` to `T::Associated` with `T:Config` bound
    let mut replacer = AssociatedTypeReplacer {
        generic_params: HashSet::new(),
        where_predicates: HashSet::new(),
    };
    // Adding bounds: `XcqTypeInfo + 'static` for any type parameter in method sigs (generic parameter and associated type).
    let mut where_clause = Vec::new();

    for item in &decl.items {
        // Collect metadata for methods only.
        let syn::TraitItem::Fn(method) = item else { continue };

        let mut inputs = Vec::new();
        let signature = &method.sig;
        for input in &signature.inputs {
            // Exclude `self` from metadata collection.
            let syn::FnArg::Typed(typed) = input else { continue };

            let pat = &typed.pat;
            let name = quote!(#pat).to_string();
            let mut ty = typed.ty.clone();

            if has_config {
                replacer.visit_type_mut(&mut ty);
            }

            where_clause.push(get_type_param(&ty));

            inputs.push(quote!(
                #xcq_primitives::metadata_ir::MethodParamMetadataIR {
                    name: #name,
                    ty: #umbrella::xcq_types::meta_type::<#ty>(),
                }
            ));
        }

        let output = match &signature.output {
            syn::ReturnType::Default => quote!(#umbrella::xcq_types::meta_type::<()>()),
            syn::ReturnType::Type(_, ty) => {
                let mut ty = ty.clone();
                if has_config {
                    replacer.visit_type_mut(&mut ty);
                }
                where_clause.push(get_type_param(&ty));
                quote!(#umbrella::xcq_types::meta_type::<#ty>())
            }
        };

        let method_name = signature.ident.to_string();

        let attrs = &method.attrs;
        methods.push(quote!(
            #( #attrs )*
            #xcq_primitives::metadata_ir::MethodMetadataIR {
                name: #method_name,
                inputs: #umbrella::xcq_types::vec![ #( #inputs, )* ],
                output: #output,
            }
        ));
    }

    let trait_name_ident = &decl.ident;
    let trait_name = trait_name_ident.to_string();
    let attrs = &decl.attrs;

    // Assume no generics
    // extract associated types
    let mut generics = Generics {
        lt_token: Some(syn::token::Lt::default()),
        params: replacer.generic_params.into_iter().collect(),
        gt_token: Some(syn::token::Gt::default()),
        where_clause: None,
    };

    for where_predicate in replacer.where_predicates {
        generics.make_where_clause().predicates.push(where_predicate);
    }
    where_clause
        .into_iter()
        .map(|ty| parse_quote!(#ty: #umbrella::xcq_types::XcqTypeInfo + 'static))
        .for_each(|w| generics.make_where_clause().predicates.push(w));

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    if has_config {
        Ok(quote!(
            #( #attrs )*
            #[inline(always)]
            pub fn runtime_metadata #impl_generics () -> #xcq_primitives::metadata_ir::ExtensionMetadataIR
            #where_clause
            {
                #xcq_primitives::metadata_ir::ExtensionMetadataIR {
                    name: #trait_name,
                    methods: #xcq_primitives::umbrella::xcq_types::vec![ #( #methods, )* ],
                }
            }
        ))
    } else {
        Ok(quote!(
            #(#attrs)*
            #[inline(always)]
            pub fn runtime_metadata() -> #xcq_primitives::metadata_ir::ExtensionMetadataIR {
                #xcq_primitives::metadata_ir::ExtensionMetadataIR {
                    name: #trait_name,
                    methods: #xcq_primitives::umbrella::xcq_types::vec![ #( #methods, )* ],
                }
            }
        ))
    }
}

/// Implement the `runtime_metadata` function on the extensions impl that
/// generates the metadata for the given traits.
/// The metadata of each extension trait is extracted from the generic function
/// exposed by `generate_decl_metadata`.
pub fn generate_impl_metadata(impls: &[ItemImpl]) -> Result<TokenStream2> {
    if impls.is_empty() {
        return Ok(quote!());
    }

    let xcq_primitives = generate_crate_access("xcq-primitives")?;

    // Get the name of the runtime for which the traits are implemented.
    let extension_impl_name = &impls
        .first()
        .expect("Traits should contain at least one implementation; qed")
        .self_ty;

    let mut metadata = Vec::new();

    for impl_ in impls {
        let mut trait_ = extract_impl_trait(impl_, RequireQualifiedTraitPath::Yes)?.clone();

        // Implementation traits are always references with a path `impl client::Core<generics> ...`
        // The trait name is the last segment of this path.
        let trait_name_ident = &trait_
            .segments
            .last()
            .as_ref()
            .expect("Trait path should always contain at least one item; qed")
            .ident;

        // Convert associated types to generics
        let mut generic_params = HashSet::<GenericParam>::new();
        for item in &impl_.items {
            if let syn::ImplItem::Type(associated_type) = item {
                let ty = &associated_type.ty;
                generic_params.insert(parse_quote!(#ty));
            }
        }
        let generics = Generics {
            lt_token: Some(syn::token::Lt::default()),
            params: generic_params.into_iter().collect(),
            gt_token: Some(syn::token::Gt::default()),
            where_clause: None,
        };

        // Extract the generics from the trait to pass to the `runtime_metadata` given by `generate_decl_metadata`
        // let generics = trait_
        //     .segments
        //     .iter()
        //     .find_map(|segment| {
        //         if let syn::PathArguments::AngleBracketed(generics) = &segment.arguments {
        //             Some(generics.clone())
        //         } else {
        //             None
        //         }
        //     })
        //     .expect("Trait path should always contain at least one generic parameter; qed");

        let mod_name = generate_mod_name_for_trait(trait_name_ident);
        // Get absolute path to the `runtime_decl_for_` module by replacing the last segment.
        if let Some(segment) = trait_.segments.last_mut() {
            *segment = parse_quote!(#mod_name);
        }

        let attrs = &impl_.attrs;
        metadata.push(quote!(
            #( #attrs )*
            #trait_::runtime_metadata::#generics()
        ));
    }

    Ok(quote!(
        impl #extension_impl_name {
            pub fn runtime_metadata() -> #xcq_primitives::metadata_ir::MetadataIR {
                #xcq_primitives::metadata_ir::MetadataIR {
                    extensions: #xcq_primitives::umbrella::xcq_types::vec![ #( #metadata, )* ],
                }
            }
        }
    ))
}

// Convert associated type to generic type
// i.e `<Self::Config as Config>::Associated` to `<Config as Config>::Associated` with `Config::Config` bound
struct AssociatedTypeReplacer {
    generic_params: HashSet<GenericParam>,
    where_predicates: HashSet<syn::WherePredicate>,
}
impl VisitMut for AssociatedTypeReplacer {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        if path.segments.len() == 2 && path.segments[0].ident == "Self" {
            let mut new_segments: Punctuated<syn::PathSegment, Token![::]> = Punctuated::new();
            let segment = &path.segments[1];
            let generic_param = format_ident!("GenericFor{}", segment.ident);
            let new_segment = syn::PathSegment {
                ident: generic_param,
                arguments: segment.arguments.clone(),
            };
            self.generic_params.insert(parse_quote!(#new_segment));
            self.where_predicates.insert(parse_quote!(#new_segment: #segment));
            new_segments.push(new_segment);
            path.segments = new_segments;
        }
    }
}

/// Instead of returning `&'a AccountId` for the first parameter, this function
/// returns `AccountId` to place bounds around it.
fn get_type_param(ty: &syn::Type) -> syn::Type {
    // Remove the lifetime and mutability of the type T to
    // place bounds around it.
    let ty_elem = match &ty {
        syn::Type::Reference(reference) => &reference.elem,
        syn::Type::Ptr(ptr) => &ptr.elem,
        syn::Type::Slice(slice) => &slice.elem,
        syn::Type::Array(arr) => &arr.elem,
        _ => ty,
    };

    ty_elem.clone()
}
