mod attributes;
mod cases;
mod macros;
mod utils;

use attributes::container::EnumAttribute;
use darling::FromDeriveInput;
use proc_macro2::Span;
use syn::DeriveInput;

use macros::{variant_count::inner_variant_count, variant_names::inner_variant_names};
use utils::unwrap_attrs;

pub(crate) fn not_an_enum() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro is only supported for enums.")
}

#[proc_macro_derive(EnumVariantCount)]
pub fn variant_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let out = inner_variant_count(&ast).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumVariantNames, attributes(delve))]
pub fn variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let attrs = unwrap_attrs!(EnumAttribute::from_derive_input(&ast));

    let out = inner_variant_names(&ast, attrs).unwrap_or_else(|err| err.to_compile_error());

    out.into()
}

#[proc_macro_derive(EnumFromString, attributes(delve))]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

#[proc_macro_derive(EnumFields, attributes(delve))]
pub fn variant_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

#[proc_macro_derive(EnumTuples)]
pub fn variant_tuples(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

#[proc_macro_derive(EnumToString, attributes(delve))]
pub fn to_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

#[proc_macro_derive(EnumHasVariant)]
pub fn has_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

#[proc_macro_derive(EnumDiscriminant)]
pub fn discriminant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}
