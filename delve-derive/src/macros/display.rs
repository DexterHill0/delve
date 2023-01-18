use deluxe::ParseAttributes;
//use proc_macro::{Ident, Span};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attributes::{
        container::EnumAttribute,
        variant::{DisplayValue, VariantAttribute},
    },
    utils::{syn_err, unwrap_attrs},
};

pub(crate) fn inner_display(
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
        let vattrs = unwrap_attrs!(VariantAttribute::parse_attributes(&variant.attrs)?);

        if vattrs.skip {
            continue;
        }

        let vname = &variant.ident;

        let (raw_fields, fields) = match &variant.fields {
            Fields::Unit => (vec![], quote! {}),
            Fields::Unnamed(un) => {
                let named = un
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("_{}", i), Span::call_site()))
                    .collect::<Vec<_>>();

                (named.clone(), quote! { ( #(#named),* ) })
            }
            Fields::Named(n) => {
                let named = n
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect::<Vec<_>>();

                (named.clone(), quote! { { #(#named),* } })
            }
        };

        match vattrs.display {
            Some(DisplayValue::String(s)) => cases.push(quote! {
                #name::#vname {..} => {
                    f.pad(#s)
                }
            }),
            Some(DisplayValue::ExternFn(_fn)) => cases.push(quote! {
                #name::#vname #fields => {
                    f.pad( &(#_fn) (#(#raw_fields),*))
                }
            }),
            Some(DisplayValue::Closure(c)) => cases.push(quote! {
                #name::#vname #fields => {
                    f.pad( &(#c) (#(#raw_fields),*))
                }
            }),
            None => {
                let field = match vattrs.rename_variant {
                    Some(name) => name,
                    _ => match eattrs.rename_variants.or_else(|| eattrs.rename_all) {
                        Some(ref inflection) => inflection.apply(&variant.ident.to_string()),
                        _ => variant.ident.to_string(),
                    },
                };

                cases.push(quote! {
                    #name::#vname {..} => {
                        f.pad(#field)
                    }
                })
            }
        };
    }

    Ok(quote! {
        impl ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    #(
                        #cases,
                    )*
                }
            }
        }
    })
}
