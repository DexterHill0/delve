use std::collections::HashMap;

use deluxe::ParseAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Type};

use crate::{
    attributes::{
        container::EnumAttribute, field::FieldAttribute, tuple::TupleAttribute,
        variant::VariantAttribute,
    },
    utils::syn_err,
};

pub(crate) fn inner_modify_variant(
    input: &DeriveInput,
    eattrs: EnumAttribute,
) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => syn_err!("This macro is only supported for enums."),
    };

    let name = &input.ident;

    // the outer hashmap stores the types that are within the tuple variant
    // each unique type needs its own impl, but only 1, hence the hashmap
    // the inner hashmap holds stores the variant name and the index(es) of the given type in the tuple variant
    // for example, if the variant is `Foo(String, usize, String)`, and the current type is `String`,
    // the out hashmap will store a `String` key, the inner hashmap will store a `Foo` key, and `vec![0, 2]`, since `String` is at indexes 0 and 2

    // `(usize, Vec<usize>)` -> `(total fields, type indexes)`
    let mut unnamed_map: HashMap<&Type, HashMap<&Ident, (usize, Vec<usize>)>> = HashMap::new();

    // `(Vec<Ident>, Vec<String>)` -> `(struct field idents, struct field idents with inflections applied)`
    let mut named_map: HashMap<&Type, HashMap<&Ident, (Vec<Ident>, Vec<String>)>> = HashMap::new();

    let mut resolved: HashMap<syn::Type, syn::Type> = HashMap::new();

    for (left, right) in eattrs.resolve {
        resolved.insert(left, right);
    }

    for variant in variants {
        let vname = &variant.ident;

        let vattrs = VariantAttribute::parse_attributes(&variant.attrs)?;

        if vattrs.skip {
            continue;
        }

        match &variant.fields {
            Fields::Unit => (),

            Fields::Unnamed(unnamed) => {
                // `i`: usize,
                // `un` -> A, B, C in (A, B, C)
                for (i, un) in unnamed.unnamed.iter().enumerate() {
                    let tattrs = TupleAttribute::parse_attributes(&un.attrs)?;

                    if tattrs.skip {
                        continue;
                    }

                    let ty = resolved.get(&un.ty).unwrap_or(&un.ty);

                    unnamed_map
                        .entry(&ty)
                        .and_modify(|v| {
                            v.entry(&vname)
                                .and_modify(|v| v.1.push(i))
                                .or_insert_with(|| (unnamed.unnamed.len(), vec![i]));
                        })
                        .or_insert_with(|| {
                            HashMap::from([(vname, (unnamed.unnamed.len(), vec![i]))])
                        });
                }
            }
            Fields::Named(named) => {
                for n in &named.named {
                    let fattrs = FieldAttribute::parse_attributes(&n.attrs)?;

                    if fattrs.skip {
                        continue;
                    }

                    let name = match fattrs.rename_field {
                        Some(name) => name,
                        _ => match eattrs.rename_fields.or_else(|| eattrs.rename_all) {
                            Some(ref inflection) => {
                                inflection.apply(&n.ident.clone().unwrap().to_string())
                            }
                            _ => n.ident.clone().unwrap().to_string(),
                        },
                    };

                    let ty = resolved.get(&n.ty).unwrap_or(&n.ty);

                    named_map
                        .entry(&ty)
                        .and_modify(|v| {
                            v.entry(&vname)
                                .and_modify(|v| {
                                    v.0.push(n.ident.clone().unwrap());
                                    v.1.push(name.clone());
                                })
                                .or_insert_with(|| {
                                    (vec![n.ident.clone().unwrap()], vec![name.clone()])
                                });
                        })
                        .or_insert_with(|| {
                            HashMap::from([(vname, (vec![n.ident.clone().unwrap()], vec![name]))])
                        });
                }
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut impls = vec![];

    for (ty, fields) in unnamed_map {
        let mut variants = vec![];
        let mut tuple_indexes = vec![];
        let mut tuple_args = vec![];
        let mut captured_args = vec![];

        for (var, indexes) in &fields {
            variants.push(var);
            tuple_indexes.push(&indexes.1);

            let mut idents = vec![];
            let mut captured = vec![];

            for i in 0..(indexes.0) {
                let ident = format_ident!("_{}", i);

                idents.push(ident.clone());

                if indexes.1.contains(&i) {
                    captured.push(ident);
                }
            }

            tuple_args.push(idents);
            captured_args.push(captured);
        }

        impls.push(quote! {
            impl #impl_generics ::delve::ModifyField<usize, #ty> for #name #ty_generics #where_clause {
                fn get_field(&self, field: usize) -> Option<&#ty> {
                    match self {
                        #(
                            #name::#variants(#(ref #tuple_args,)*) => match field {
                                #(
                                    #tuple_indexes => Some(#captured_args),
                                )*
                                _ => None,
                            }
                        )*
                        _ => None,
                    }
                }

                fn set_field(&mut self, field: usize, value: #ty) -> Option<()> {
                    match self {
                        #(
                            #name::#variants(#(ref mut #tuple_args,)*) => match field {
                                #(
                                    #tuple_indexes => {
                                        *#captured_args = value;
                                        Some(())
                                    },
                                )*
                                _ => None,
                            }
                        )*
                        _ => None,
                    }
                }
            }
        })
    }

    for (ty, fields) in named_map {
        let mut variants = vec![];
        let mut field_names = vec![];
        let mut field_args = vec![];

        // keep it in order
        for (var, names) in &fields {
            variants.push(var);
            field_names.push(&names.0);

            field_args.push(&names.1);
        }

        impls.push(quote! {
            impl #impl_generics ::delve::ModifyField<&str, #ty> for #name #ty_generics #where_clause {
                fn get_field(&self, field: &str) -> Option<&#ty> {
                    match self {
                        #(
                            #name::#variants { #(ref #field_names,)* .. } => match &field[..] {
                                #(
                                    #field_args => Some(#field_names),
                                )*
                                _ => None,
                            }
                        )*
                        _ => None,
                    }
                }

                fn set_field(&mut self, field: &str, value: #ty) -> Option<()> {
                    match self {
                        #(
                            #name::#variants { #(ref mut #field_names,)* .. } => match &field[..] {
                                #(
                                    #field_args => {
                                        *#field_names = value;
                                        Some(())
                                    },
                                )*
                                _ => None,
                            }
                        )*
                        _ => None,
                    }
                }
            }
        })
    }

    Ok(quote! {
        #(#impls)*
    })
}
