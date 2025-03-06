use super::helper;
use crate::utils::generate_crate_access;
use frame_support_procedural_tools::get_doc_literals;
use inflector::Inflector;
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
    /// Information on methods
    pub methods: Vec<ExtensionMethod>,
    /// The where clause of the trait
    pub where_clause: Option<syn::WhereClause>,
    /// The span of the extension_decl::view_fns attribute
    pub attr_span: proc_macro2::Span,
    /// docs on the trait
    pub docs: Vec<syn::Expr>,
    /// attributes on the trait
    pub attrs: Vec<syn::Attribute>,
}

/// Definition of a view fn variant
pub struct ExtensionMethod {
    /// Function name
    pub name: syn::Ident,
    /// Information on args: `(name, type)`
    pub args: Vec<(syn::Ident, Box<syn::Type>)>,
    /// The return type of the view_fn
    pub return_type: syn::ReturnType,
    /// docs on the view_fn
    pub docs: Vec<syn::Expr>,
    /// Attributes of the view_fn
    pub attrs: Vec<syn::Attribute>,
}

/// Attributes for functions in view fns
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
    pub fn try_from(module_name: &str, item: &mut syn::Item) -> syn::Result<Self> {
        let parity_scale_codec = generate_crate_access("parity_scale_codec")?;
        let scale_info = generate_crate_access("scale_info")?;
        let syn::Item::Trait(item) = item else {
            let msg = "Invalid extension_decl::extension, expected trait definition";
            return Err(syn::Error::new(item.span(), msg));
        };
        if !matches!(item.vis, syn::Visibility::Public(_)) {
            let msg = "Invalid extension_decl::extension, expected public trait definition";
            return Err(syn::Error::new(item.span(), msg));
        }

        if item.ident.to_string().to_snake_case() != module_name {
            let msg = "Invalid extension_decl::extension, expected the snake case version of the trait's ident to match the module name, e.g. `ExtensionCore` matches `extension_core`";
            return Err(syn::Error::new(item.ident.span(), msg));
        }

        if !item.generics.params.is_empty() {
            let msg = "Invalid extension_decl::extension, expected no generics";
            return Err(syn::Error::new(item.generics.params[0].span(), msg));
        }

        let mut methods = vec![];
        let mut indices = HashMap::new();
        let mut last_index: Option<u8> = None;
        for item in &mut item.items {
            match item {
                syn::TraitItem::Type(item_type) => {
                    // Add `Codec + TypeInfo + 'static` to the bound of associated types
                    item_type
                        .bounds
                        .push(syn::parse_quote!(#parity_scale_codec::Codec + #scale_info::TypeInfo + 'static));
                }
                syn::TraitItem::Fn(method) => {
                    let mut view_fn_index_attrs = vec![];
                    for attr in helper::take_item_extension_decl_attrs(&mut method.attrs)?.into_iter() {
                        match attr {
                            FunctionAttr::FnIndex(index) => {
                                view_fn_index_attrs.push(attr);
                            }
                        }
                    }

                    if view_fn_index_attrs.len() > 1 {
                        let msg = "Invalid extension_decl::extension, too many fn_index attributes given";
                        return Err(syn::Error::new(method.sig.span(), msg));
                    }

                    let view_fn_index = view_fn_index_attrs.pop().map(|attr| match attr {
                        FunctionAttr::FnIndex(index) => index,
                        _ => unreachable!("checked during parsing"),
                    });
                    let explicit_view_fn_index = view_fn_index.is_some();

                    let final_index = match view_fn_index {
                        Some(i) => i,
                        None => last_index.map_or(Some(0), |idx| idx.checked_add(1)).ok_or_else(|| {
                            let msg = "Function index doesn't fit into u8, index is 256";
                            syn::Error::new(method.sig.span(), msg)
                        })?,
                    };
                    last_index = Some(final_index);

                    if let Some(used_fn) = indices.insert(final_index, method.sig.ident.clone()) {
                        let error_msg = format!(
                            "Invalid extension_decl::view_fns; Both functions {} and {} are at index {}",
                            used_fn, method.sig.ident, final_index
                        );
                        let mut err = syn::Error::new(used_fn.span(), &error_msg);
                        err.combine(syn::Error::new(method.sig.ident.span(), &error_msg));
                        return Err(err);
                    }

                    let mut args = vec![];
                    for arg in method.sig.inputs.iter() {
                        let arg = if let syn::FnArg::Typed(arg) = arg {
                            arg
                        } else {
                            let msg = "Invalid extension_decl::extension, every argument should be typed instead of receiver(self)";
                            return Err(syn::Error::new(arg.span(), msg));
                        };
                        let arg_ident = if let syn::Pat::Ident(pat) = &*arg.pat {
                            pat.ident.clone()
                        } else {
                            let msg = "Invalid extension_decl::view fns, argument must be ident";
                            return Err(syn::Error::new(arg.pat.span(), msg));
                        };
                        args.push((arg_ident, arg.ty.clone()))
                    }

                    let docs = get_doc_literals(&method.attrs);

                    methods.push(ExtensionMethod {
                        name: method.sig.ident.clone(),
                        args,
                        return_type: method.sig.output.clone(),
                        docs,
                        attrs: method.attrs.clone(),
                    });
                }
                _ => {}
            }
        }

        Ok(Self {
            name: item.ident.clone(),
            methods,
            where_clause: item.generics.where_clause.clone(),
            attr_span: item.span(),
            docs: get_doc_literals(&item.attrs),
            attrs: item.attrs.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn no_generics_works() {
        let mut item = parse_str::<syn::ItemTrait>("pub trait ExtensionCore { fn foo(self) -> ExtensionId; }").unwrap();
        let result = Extension::try_from("extension_core", &mut item);
        assert!(result.is_ok());
    }

    #[test]
    fn single_generic_with_config_bound_works() {
        let mut item =
            parse_str::<syn::ItemTrait>("pub trait ExtensionCore<T: Config> { fn foo(self) -> T::ExtensionId; }")
                .unwrap();
        let result = Extension::try_from("extension_core", &mut item);
        assert!(result.is_ok());
    }

    #[test]
    fn has_no_more_than_one_generic() {
        let mut item = parse_str::<syn::ItemTrait>(
            "pub trait ExtensionCore<ExtensionId, AccountId> { fn foo(self) -> ExtensionId; }",
        )
        .unwrap();
        let result = Extension::try_from("extension_core", &mut item);
        assert!(result.is_err());
    }

    #[test]
    fn no_items() {
        let mut item = parse_str::<syn::ItemTrait>("pub trait ExtensionCore { }").unwrap();
        let result = Extension::try_from("extension_core", &mut item);
        assert!(result.is_err());
    }

    #[test]
    fn single_generic_has_no_where_clause() {
        let mut item = parse_str::<syn::ItemTrait>("pub trait ExtensionCore<A> { fn foo(self) -> A; }").unwrap();
        let result = Extension::try_from("extension_core", &mut item);
        assert!(result.is_err());
    }
}
