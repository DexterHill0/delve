use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::utils::syn_err;

pub(crate) fn inner_has_variant(input: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => syn_err!("This macro is only supported for enums."),
    };

    let names = variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect::<Vec<_>>();

    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::delve::HasVariant for #name #ty_generics #where_clause {
            fn has_variant(variant: &str) -> bool {
                match variant {
                    #(
                        #names => true,
                    )*
                    _ => false,
                }
            }
        }
    })
}
