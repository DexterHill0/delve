use deluxe::ParseAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attributes::{container::EnumAttribute, variant::VariantAttribute},
    utils::syn_err,
};

pub(crate) fn inner_from_str(
    input: &DeriveInput,
    eattrs: EnumAttribute,
) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => syn_err!("This macro is only supported for enums."),
    };

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut cases = vec![];

    for variant in variants {
        let vattrs = VariantAttribute::parse_attributes(&variant.attrs)?;

        if vattrs.skip {
            continue;
        }

        let from = if vattrs.from.is_empty() {
            vec![match vattrs.rename_variant {
                Some(name) => name,
                _ => match eattrs.rename_variants.or_else(|| eattrs.rename_all) {
                    Some(ref inflection) => inflection.apply(&variant.ident.to_string()),
                    _ => variant.ident.to_string(),
                },
            }]
        } else {
            vattrs.from
        };

        let params = match &variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(fields) => {
                let defaults = ::core::iter::repeat(quote!(::core::default::Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: ::core::default::Default::default()),*} }
            }
        };

        let vname = &variant.ident;

        for f in from {
            if !vattrs.ascii_case_insensitive {
                cases.push(quote! {
                    #f => ::core::result::Result::Ok(#name::#vname #params)
                });
            } else {
                cases.push(quote! {
                    s if s.eq_ignore_ascii_case(#f) => ::core::result::Result::Ok(#name::#vname #params)
                });
            }
        }
    }

    Ok(quote! {
        impl #impl_generics ::core::str::FromStr for #name #ty_generics #where_clause {
            type Err = ::delve::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(
                        #cases,
                    )*
                   _ => ::core::result::Result::Err(::delve::ParseError::VariantNotFound)
                }
            }
        }
    })
}
