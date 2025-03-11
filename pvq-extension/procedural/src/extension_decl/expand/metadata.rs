use crate::extension_decl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::visit_mut::VisitMut;

/// Generate the runtime metadata of the provided extension trait.
pub fn expand_metadata(def: &Def) -> TokenStream2 {
    let pvq_extension = &def.pvq_extension;
    let scale_info = &def.scale_info;

    let mut functions = Vec::new();

    let mut replacer = AssociatedTypeReplacer;

    for function in &def.extension.functions {
        let mut inputs = Vec::new();
        for (name, ty) in &function.inputs {
            let name = name.to_string();
            let mut ty = ty.clone();

            // Replace Self::AssociatedType with Impl::AssociatedType
            replacer.visit_type_mut(&mut ty);

            inputs.push(quote!(
                #pvq_extension::metadata::FunctionParamMetadata {
                    name: #name,
                    ty: #scale_info::meta_type::<#ty>(),
                }
            ));
        }

        let output = match &function.output {
            syn::ReturnType::Default => quote!(#scale_info::meta_type::<()>()),
            syn::ReturnType::Type(_, ty) => {
                let mut ty = ty.clone();
                replacer.visit_type_mut(&mut ty);
                quote!(#scale_info::meta_type::<#ty>())
            }
        };

        let function_name = function.name.to_string();

        functions.push(quote!(
            #pvq_extension::metadata::FunctionMetadata {
                name: #function_name,
                inputs: #scale_info::prelude::vec![ #( #inputs, )* ],
                output: #output,
            }
        ));
    }

    let trait_ident = &def.extension.name;
    let trait_name = trait_ident.to_string();
    let metadata = if def.extension.types.is_empty() {
        quote!(
            pub fn metadata () -> #pvq_extension::metadata::ExtensionMetadata {
                #pvq_extension::metadata::ExtensionMetadata {
                    name: #trait_name,
                    functions: #scale_info::prelude::vec![ #( #functions, )* ],
                }
            }
        )
    } else {
        let impl_generics = quote!(Impl: #trait_ident);
        quote!(
            pub fn metadata <#impl_generics> () -> #pvq_extension::metadata::ExtensionMetadata {
                #pvq_extension::metadata::ExtensionMetadata {
                    name: #trait_name,
                    functions: #scale_info::prelude::vec![ #( #functions, )* ],
                }
            }
        )
    };
    metadata
}

// Convert `Self::AssociatedType` to `Impl::AssociatedType`
struct AssociatedTypeReplacer;
impl syn::visit_mut::VisitMut for AssociatedTypeReplacer {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        if path.segments.len() == 2 && path.segments[0].ident == "Self" {
            path.segments[0].ident = syn::Ident::new("Impl", path.segments[0].ident.span());
        }
    }
}
