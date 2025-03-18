use syn::spanned::Spanned;
mod extension_fn;
pub use extension_fn::ExtensionFn;
mod entrypoint;
pub use entrypoint::EntrypointDef;
mod helper;
// program definition
pub struct Def {
    pub item: syn::ItemMod,
    pub extension_fns: Vec<ExtensionFn>,
    pub entrypoint: EntrypointDef,
    pub parity_scale_codec: syn::Path,
    pub polkavm_derive: syn::Path,
}

impl Def {
    pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
        let parity_scale_codec = helper::generate_crate_access("parity-scale-codec")?;
        let polkavm_derive = helper::generate_crate_access("polkavm-derive")?;
        let mod_span = item.span();
        let items = &mut item
            .content
            .as_mut()
            .ok_or_else(|| {
                let msg = "Invalid #[program] definition, expected mod to be inline.";
                syn::Error::new(mod_span, msg)
            })?
            .1;

        let mut extension_fns = Vec::new();
        let mut entrypoint = None;

        let mut i = 0;
        while i < items.len() {
            let item = &mut items[i];
            if let Some(attr) = helper::take_first_program_attr(item)? {
                if let Some(last_segment) = attr.path().segments.last() {
                    if last_segment.ident == "extension_fn" {
                        let mut extension_id = None;
                        let mut fn_index = None;
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("extension_id") {
                                let value = meta.value()?;
                                extension_id = Some(value.parse::<syn::LitInt>()?.base10_parse::<u64>()?);
                            } else if meta.path.is_ident("fn_index") {
                                let value = meta.value()?;
                                fn_index = Some(value.parse::<syn::LitInt>()?.base10_parse::<u32>()?);
                            } else {
                                return Err(syn::Error::new(
                                    meta.path.span(),
                                    "Invalid attribute meta, expected `extension_id` or `fn_index`",
                                ));
                            }
                            Ok(())
                        })?;

                        let removed_item = items.remove(i);
                        let extension_fn = ExtensionFn::try_from(attr.span(), removed_item, extension_id, fn_index)?;
                        extension_fns.push(extension_fn);
                        continue;
                    } else if last_segment.ident == "entrypoint" {
                        if entrypoint.is_some() {
                            return Err(syn::Error::new(attr.span(), "Only one entrypoint function is allowed"));
                        }
                        entrypoint = Some(EntrypointDef::try_from(attr.span(), item)?);
                        continue;
                    } else {
                        return Err(syn::Error::new(
                            item.span(),
                            "Invalid attribute, expected `#[program::extension_fn]` or `#[program::entrypoint]`",
                        ));
                    }
                }
            }
            i += 1;
        }

        let entrypoint =
            entrypoint.ok_or_else(|| syn::Error::new(mod_span, "At least one entrypoint function is required"))?;
        let def = Def {
            item,
            extension_fns,
            entrypoint,
            parity_scale_codec,
            polkavm_derive,
        };

        Ok(def)
    }
}

/// List of additional token to be used for parsing.
mod keyword {
    syn::custom_keyword!(program);
    syn::custom_keyword!(extension_id);
    syn::custom_keyword!(fn_index);
    syn::custom_keyword!(entrypoint);
}
