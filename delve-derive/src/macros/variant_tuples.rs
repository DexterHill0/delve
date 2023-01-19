use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::utils::syn_err;

pub(crate) fn inner_variant_tuples(input: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => syn_err!("This macro is only supported for enums."),
    };

    let name = &input.ident;

    let mut cases = vec![];

    for variant in variants {
        let vname = &variant.ident;

        match &variant.fields {
            Fields::Unit => (),
            Fields::Unnamed(un) => {
                let count = un.unnamed.len();
                cases.push(quote! {
                    #name::#vname {..} => Some(#count)
                });
            }
            Fields::Named(_) => (),
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::delve::TupleCount for #name #ty_generics #where_clause {
            fn tuple_count(&self) -> Option<usize> {
                match self {
                    #(
                        #cases,
                    )*
                    _ => None,
                }
            }
        }
    })
}
