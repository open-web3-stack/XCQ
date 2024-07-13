use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Field, Fields};

#[proc_macro_derive(XcqTypeInfo)]
pub fn derive_xcq_type_info(input: TokenStream) -> TokenStream {
    match generate(input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate(input: TokenStream2) -> syn::Result<TokenStream2> {
    let type_info_impl: XcqTypeInfoImpl = XcqTypeInfoImpl::parse(input)?;
    let type_info_impl_toks = type_info_impl.expand()?;
    Ok(quote! {
        // A rust pattern to ensure that the type info is implemented.
        #[allow(non_upper_case_globals, unused_attributes, unused_imports)]
        const _: () = {
            #type_info_impl_toks
        };
    })
}

struct XcqTypeInfoImpl {
    ast: DeriveInput,
}

impl XcqTypeInfoImpl {
    fn parse(input: TokenStream2) -> syn::Result<Self> {
        let ast = syn::parse2::<DeriveInput>(input)?;
        // Assume no attributes
        if !ast.attrs.is_empty() {
            return Err(syn::Error::new_spanned(ast, "unexpected attributes"));
        }
        Ok(Self { ast })
    }

    fn expand(&self) -> syn::Result<TokenStream2> {
        let xcq_types = import_xcq_types();
        let ident = &self.ast.ident;

        // Assume no generics
        if self.ast.generics.type_params().next().is_some() {
            return Err(syn::Error::new_spanned(
                &self.ast.generics,
                "generics are not supported",
            ));
        }

        let type_info = match &self.ast.data {
            Data::Struct(ref s) => self.generate_struct_type(s),
            Data::Enum(ref e) => self.generate_enum_type(e),
            Data::Union(_) => {
                return Err(syn::Error::new_spanned(&self.ast, "unions are not supported"));
            }
        };

        // TODO: No token replacement supported yet
        Ok(quote! {
            impl #xcq_types::XcqTypeInfo for #ident {
                type Identity = Self;
                fn type_info() -> #xcq_types::XcqType {
                    #type_info
                }
            }
        })
    }

    fn generate_struct_type(&self, data_struct: &DataStruct) -> TokenStream2 {
        let xcq_types = import_xcq_types();
        let ident = &self.ast.ident;
        let fields = match data_struct.fields {
            Fields::Named(ref fields) => self.generate_fields(&fields.named),
            Fields::Unnamed(ref fields) => self.generate_fields(&fields.unnamed),
            Fields::Unit => return quote! {},
        };

        quote! {
            #xcq_types::StructType {
                ident: stringify!(#ident).as_bytes().to_vec(),
                fields: vec![#(#fields),*],
            }.into()
        }
    }

    fn generate_enum_type(&self, data_enum: &DataEnum) -> TokenStream2 {
        let xcq_types = import_xcq_types();
        let ident = &self.ast.ident;
        let variants = data_enum.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let fields = match variant.fields {
                Fields::Named(ref fields) => self.generate_fields(&fields.named),
                Fields::Unnamed(ref fields) => self.generate_fields(&fields.unnamed),
                Fields::Unit => return quote! {},
            };
            quote! {
                #xcq_types::Variant {
                    ident: stringify!(#ident).as_bytes().to_vec(),
                    fields: vec![#(#fields),*],
                }
            }
        });
        quote! {
            #xcq_types::EnumType {
                ident: stringify!(#ident).as_bytes().to_vec(),
                variants: vec![#(#variants),*],
            }.into()
        }
    }

    fn generate_fields(&self, fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
        let xcq_types = import_xcq_types();
        fields
            .iter()
            .map(|f| {
                let ty = &f.ty;
                let ident_toks = match &f.ident {
                    Some(ident) => quote! { stringify!(#ident).as_bytes().to_vec()},
                    None => quote! { vec![] },
                };
                quote! {
                    #xcq_types::Field {
                        ident: #ident_toks,
                        ty: <#ty as #xcq_types::XcqTypeInfo>::type_info(),
                    }
                }
            })
            .collect()
    }
}

fn import_xcq_types() -> TokenStream2 {
    let found_crate = crate_name("xcq-types").expect("xcq-types not found in Cargo.toml");
    match found_crate {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let crate_ = syn::Ident::new(&name.replace('-', "_"), proc_macro2::Span::call_site());
            quote! { ::#crate_ }
        }
    }
}
