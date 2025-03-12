use super::helper;
use crate::utils::generate_crate_access;
use std::collections::HashMap;
use syn::spanned::Spanned;

/// List of additional token to be used for parsing
mod keyword {
    syn::custom_keyword!(extension_decl);
    syn::custom_keyword!(fn_index);
}

/// Definition of a ViewFns trait
/// #[extension_decl::extension]
/// pub trait ExtensionCore {
///     type ExtensionId: Codec;
///     #[extension_decl::fn_index(expr)]
///     fn has_extension(id: Self::ExtensionId) -> bool;
/// }
pub struct Extension {
    /// The name of the trait
    pub name: syn::Ident,
    /// The associated type of the trait
    pub types: Vec<ExtensionType>,
    /// Information on functions
    pub functions: Vec<ExtensionFunction>,
}

#[derive(Debug)]
pub struct ExtensionType {
    #[allow(dead_code)]
    pub name: syn::Ident,
}

/// Definition of a function variant
pub struct ExtensionFunction {
    /// Function name
    pub name: syn::Ident,
    /// Information on inputs: `(name, type)`
    pub inputs: Vec<(syn::Ident, Box<syn::Type>)>,
    /// The return type of the function
    pub output: syn::ReturnType,
}

/// Attributes for functions
pub enum FunctionAttr {
    /// Parse for #[extension::fn_index(expr)]
    FnIndex(u8),
}

impl syn::parse::Parse for FunctionAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        content.parse::<keyword::extension_decl>()?;
        content.parse::<syn::Token![::]>()?;

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::fn_index) {
            content.parse::<keyword::fn_index>()?;
            let fn_index_content;
            syn::parenthesized!(fn_index_content in content);
            let index = fn_index_content.parse::<syn::LitInt>()?;
            if !index.suffix().is_empty() {
                let msg = "Number literal must not have a suffix";
                return Err(syn::Error::new(index.span(), msg));
            }
            Ok(Self::FnIndex(index.base10_parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Extension {
    pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
        let parity_scale_codec = generate_crate_access("parity-scale-codec")?;
        let scale_info = generate_crate_access("scale-info")?;
        let syn::Item::Trait(item) = item else {
            let msg = "Invalid extension_decl::extension, expected trait definition";
            return Err(syn::Error::new(item.span(), msg));
        };
        if !matches!(item.vis, syn::Visibility::Public(_)) {
            let msg = "Invalid extension_decl::extension, expected public trait definition";
            return Err(syn::Error::new(item.span(), msg));
        }

        if !item.generics.params.is_empty() {
            let msg = "Invalid extension_decl::extension, expected no generics";
            return Err(syn::Error::new(item.generics.params[0].span(), msg));
        }

        let mut types = vec![];
        let mut functions = vec![];
        let mut indices = HashMap::new();
        let mut last_index: Option<u8> = None;
        for item in &mut item.items {
            match item {
                syn::TraitItem::Type(item_type) => {
                    // Add `Codec + TypeInfo + 'static` bounds if not present
                    if item_type
                        .bounds
                        .iter()
                        .all(|bound| bound != &syn::parse_quote!(#parity_scale_codec::Codec))
                    {
                        item_type.bounds.push(syn::parse_quote!(#parity_scale_codec::Codec));
                    }
                    if item_type
                        .bounds
                        .iter()
                        .all(|bound| bound != &syn::parse_quote!(#scale_info::TypeInfo))
                    {
                        item_type.bounds.push(syn::parse_quote!(#scale_info::TypeInfo));
                    }
                    if item_type
                        .bounds
                        .iter()
                        .all(|bound| bound != &syn::parse_quote!('static))
                    {
                        item_type.bounds.push(syn::parse_quote!('static));
                    }
                    types.push(ExtensionType {
                        name: item_type.ident.clone(),
                    });
                }
                syn::TraitItem::Fn(function) => {
                    let mut function_index_attrs = vec![];
                    for attr in helper::take_item_extension_decl_attrs(&mut function.attrs)?.into_iter() {
                        match attr {
                            FunctionAttr::FnIndex(_) => {
                                function_index_attrs.push(attr);
                            }
                        }
                    }

                    if function_index_attrs.len() > 1 {
                        let msg = "Invalid extension_decl::extension, too many fn_index attributes given";
                        return Err(syn::Error::new(function.sig.span(), msg));
                    }

                    let function_index = function_index_attrs.pop().map(|attr| match attr {
                        FunctionAttr::FnIndex(index) => index,
                    });

                    let final_index = match function_index {
                        Some(i) => i,
                        None => last_index.map_or(Some(0), |idx| idx.checked_add(1)).ok_or_else(|| {
                            let msg = "Function index doesn't fit into u8, index is 256";
                            syn::Error::new(function.sig.span(), msg)
                        })?,
                    };
                    last_index = Some(final_index);

                    if let Some(used_fn) = indices.insert(final_index, function.sig.ident.clone()) {
                        let error_msg = format!(
                            "Invalid extension_decl::extension; Both functions {} and {} are at index {}",
                            used_fn, function.sig.ident, final_index
                        );
                        let mut err = syn::Error::new(used_fn.span(), &error_msg);
                        err.combine(syn::Error::new(function.sig.ident.span(), &error_msg));
                        return Err(err);
                    }

                    let mut inputs = vec![];
                    for input in function.sig.inputs.iter() {
                        let input = if let syn::FnArg::Typed(input) = input {
                            input
                        } else {
                            let msg = "Invalid extension_decl::extension, every input argument should be typed instead of receiver(self)";
                            return Err(syn::Error::new(input.span(), msg));
                        };
                        let input_ident = if let syn::Pat::Ident(pat) = &*input.pat {
                            pat.ident.clone()
                        } else {
                            let msg = "Invalid extension_decl::extension, input argument must be ident";
                            return Err(syn::Error::new(input.pat.span(), msg));
                        };
                        inputs.push((input_ident, input.ty.clone()))
                    }

                    functions.push(ExtensionFunction {
                        name: function.sig.ident.clone(),
                        inputs,
                        output: function.sig.output.clone(),
                    });
                }
                _ => {}
            }
        }

        if functions.is_empty() {
            let msg = "Invalid extension_decl::extension, expected at least one function";
            return Err(syn::Error::new(item.span(), msg));
        }

        Ok(Self {
            name: item.ident.clone(),
            types,
            functions,
        })
    }
}
