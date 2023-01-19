use deluxe::ParseAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::{
    attributes::{container::EnumAttribute, variant::VariantAttribute},
    utils::syn_err,
};

pub(crate) fn inner_to_str(input: &DeriveInput, eattrs: EnumAttribute) -> syn::Result<TokenStream> {
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

        let to = if vattrs.to.is_none() {
            match vattrs.rename_variant {
                Some(name) => name,
                _ => match eattrs.rename_variants.or_else(|| eattrs.rename_all) {
                    Some(ref inflection) => inflection.apply(&variant.ident.to_string()),
                    _ => variant.ident.to_string(),
                },
            }
        } else {
            vattrs.to.unwrap()
        };

        let vname = &variant.ident;

        cases.push(quote! {
            #name::#vname {..} => #to
        });
    }

    let str_name = &name.to_string();

    Ok(quote! {
        impl #impl_generics ::core::convert::From<#name #ty_generics> for &'static str #where_clause {
            fn from(value: #name #ty_generics) -> Self {
                match value {
                    #( #cases, )*
                    _ => panic!(concat!("`Into<&'static str>/From<", #str_name, "> for &'static str` called on skipped variant"))
                }
            }
        }
    })
}
