use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

use crate::{
    attributes::{container::EnumAttribute, variant::VariantAttribute},
    not_an_enum,
    utils::unwrap_attrs,
};

pub(crate) fn inner_variant_names(
    input: &DeriveInput,
    eattrs: EnumAttribute,
) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(not_an_enum()),
    };

    let mut names = vec![];

    for variant in variants {
        let vattrs = unwrap_attrs!(VariantAttribute::from_attributes(&variant.attrs)?);

        if vattrs.skip.is_some() {
            continue;
        }

        match vattrs.rename_variant {
            Some(ref name) => {
                names.push(name.clone());
            }
            _ => match eattrs.rename_variants {
                Some(ref inflection) => names.push(inflection.apply(&variant.ident.to_string())),
                _ => names.push(variant.ident.to_string()),
            },
        }
    }

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::delve::VariantNames for #name #ty_generics #where_clause {
            const VARIANT_NAMES: &'static [&'static str] = &[#(#names),*];
        }
    })
}
