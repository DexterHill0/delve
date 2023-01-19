use deluxe::ParseAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attributes::{container::EnumAttribute, field::FieldAttribute},
    utils::syn_err,
};

pub(crate) fn inner_variant_fields(
    input: &DeriveInput,
    eattrs: EnumAttribute,
) -> syn::Result<TokenStream> {
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
            Fields::Unnamed(_) => (),
            Fields::Named(named) => {
                let mut names = vec![];

                for n in &named.named {
                    let fattrs = FieldAttribute::parse_attributes(&n.attrs)?;

                    if fattrs.skip {
                        continue;
                    }

                    names.push(match fattrs.rename_field {
                        Some(name) => name,
                        _ => match eattrs.rename_fields.or_else(|| eattrs.rename_all) {
                            Some(ref inflection) => {
                                inflection.apply(&n.ident.clone().unwrap().to_string())
                            }
                            _ => n.ident.clone().unwrap().to_string(),
                        },
                    });
                }

                cases.push(quote! {
                    #name::#vname {..} => Some(&[#(#names),*])
                })
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::delve::FieldNames for #name #ty_generics #where_clause {
            fn field_names(&self) -> Option<&'static [&'static str]> {
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
