#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
//! This crate provides `delve`'s macros.
//!
//! Refer to [`delve`](https://docs.rs/delve) for how to use them.
//!

mod attributes;
mod cases;
mod macros;
mod utils;

use attributes::container::EnumAttribute;
use deluxe::ParseAttributes;
use syn::DeriveInput;

use macros::{
    display::inner_display, from_str::inner_from_str, has_variant::inner_has_variant,
    modify_variant::inner_modify_variant, to_str::inner_to_str, variant_count::inner_variant_count,
    variant_fields::inner_variant_fields, variant_names::inner_variant_names,
    variant_tuples::inner_variant_tuples,
};
use utils::unwrap_attrs;

#[proc_macro_derive(EnumVariantCount)]
pub fn variant_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let out = inner_variant_count(&ast).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumVariantNames, attributes(delve))]
pub fn variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_variant_names(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumFromStr, attributes(delve))]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_from_str(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumFields, attributes(delve))]
pub fn variant_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_variant_fields(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumTuples)]
pub fn variant_tuples(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let out = inner_variant_tuples(&ast).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumModify, attributes(delve))]
pub fn modify_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_modify_variant(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumToStr, attributes(delve))]
pub fn to_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_to_str(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumDisplay, attributes(delve))]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::parse_attributes(&ast.attrs));

    let out = inner_display(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumHasVariant)]
pub fn has_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let out = inner_has_variant(&ast).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}
