use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::utils::syn_err;

pub(crate) fn inner_variant_count(input: &DeriveInput) -> syn::Result<TokenStream> {
    let count = match &input.data {
        Data::Enum(v) => v.variants.len(),
        _ => syn_err!("This macro is only supported for enums."),
    };

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::delve::VariantCount for #name #ty_generics #where_clause {
            const VARIANT_COUNT: usize = #count;
        }
    })
}
