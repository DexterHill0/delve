#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![allow(non_snake_case)]

//! # `delve`
//!
//! A bunch of macros to improve working with enums and strings.
//!
//! [![github]](https://github.com/DexterHill0/delve)&ensp;[![crates-io]](https://crates.io/crates/delve)&ensp;[![docs-rs]](https://docs.rs/delve)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!

// only for documentation
pub mod ContainerAttributes;
pub mod FieldAttributes;
pub mod TupleAttributes;
pub mod VariantAttributes;

/// Implements `delve::VariantCount` which provides the number of variants within an enum.
/// See [`VariantCount`]: trait.VariantCount.html for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumVariantCount;

/// Implements `delve::VariantNames` which provides the names of all the variants within an enum.
/// See [`VariantNames`] for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumVariantNames;

/// Implements `core::str::FromStr` on an enum.
///
/// # Example
///
/// ```rust
/// use core::str::FromStr;
/// use delve::EnumFromStr;
///
/// #[derive(EnumFromStr)]
/// enum Week {
///     Sunday,
///     Monday,
///     Tuesday,
///     // ...
/// }
///
/// assert_eq!(Ok(Week::Tuesday), Week::from_str("Tuesday"));
/// ```
#[cfg(feature = "derive")]
pub use delve_derive::EnumFromStr;

/// Implements `delve::FieldNames` which provides the names of the fields within a struct variant.
/// See [`FieldNames`] for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumFields;

/// Implements `delve::TupleCount` which provides the number of type arguments in a tuple variant.
/// See [`TupleCount`] for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumTuples;

/// Implements `delve::ModifyField` which allows the modification of arguments within a tuple or struct variant.
/// See [`ModifyField`] for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumModify;

/// Implements `From<TheEnum> for &'static str` on an enum.
/// The Rust `std` provides a blanket implementation for the reverse so `Into<&'static str> for TheEnum` is also implemented.
///
/// # Example
///
/// ```rust
/// use delve::EnumToStr;
///
/// #[derive(EnumToStr)]
/// enum Week {
///     Sunday,
///     Monday,
///     Tuesday,
///     // ...
/// }
///
/// assert_eq!("Sunday", <Week as Into<&'static str>>::into(Week::Sunday));
/// ```
#[cfg(feature = "derive")]
pub use delve_derive::EnumToStr;

/// Implements `core::fmt::Display` on an enum.
///
/// # Example
///
/// ```rust
/// use delve::EnumDisplay;
///
/// #[derive(EnumDisplay)]
/// enum Week {
///     Sunday,
///     Monday,
///     Tuesday,
/// }
///
/// assert_eq!("Monday", format!("{}", Week::Monday));
/// ```
#[cfg(feature = "derive")]
pub use delve_derive::EnumDisplay;

/// Implements `delve::HasVariant` which returns whether a given variant name exists in the enum.
/// See [`HasVariant`] for an example implementation.
#[cfg(feature = "derive")]
pub use delve_derive::EnumHasVariant;

/// The error returned in `FromStr` if it failed to parse the string into a variant.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

#[cfg(feature = "std")]
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::VariantNotFound => {
                write!(f, "Variant not found matching the given string.")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::VariantNotFound => "Variant not found matching the given string.",
        }
    }
}

/// A trait for getting the number of variants in an enum.
/// Use [`delve_derive::EnumVariantCount`] to automatically derive this trait.
pub trait VariantCount {
    const VARIANT_COUNT: usize;
}

/// A trait for getting the names of the variants in an enum.
/// Use [`delve_derive::EnumVariantNames`] to automatically derive this trait.
pub trait VariantNames {
    const VARIANT_NAMES: &'static [&'static str];
}

/// A trait that returns whether a given variant name exists in the enum. Unlike checking if the variant name exists
/// using the `VariantNames` trait, this is *not* affected by inflections.
/// Use [`delve_derive::EnumHasVariant`] to automatically derive this trait.
///
/// # Example
///
/// ```rust
/// use delve::{EnumHasVariant, HasVariant};
///
/// #[derive(EnumHasVariant)]
/// enum Operators {
///     Add,
///     Div,
///     Sub,
///     Mul
/// }
///
/// assert!(Operators::has_variant("Div"));
/// ```
pub trait HasVariant {
    fn has_variant(variant: &str) -> bool;
}

/// A trait that returns the field names from within a struct variant.
/// Use [`delve_derive::EnumFields`] to automatically derive this trait.
///
/// # Example
///
/// ```rust
/// use delve::{EnumFields, FieldNames};
///
/// #[derive(EnumFields)]
/// enum Color {
///     Red,
///     Custom {
///         r: usize,
///         g: usize,
///         b: usize,
///     }
/// }
///
/// assert_eq!(Some(&["r", "g", "b"][..]), Color::Custom { r: 255, g: 50, b: 100, }.field_names());
/// ```
pub trait FieldNames {
    fn field_names(&self) -> Option<&'static [&'static str]>;
}

/// A trait that returns the number of types within a tuple variant.
/// Use [`delve_derive::EnumTuples`] to automatically derive this trait.
///
/// # Example
///
/// ```rust
/// use delve::{EnumTuples, TupleCount};
///
/// #[derive(EnumTuples)]
/// enum Week {
///     Day(String, usize),
/// }
///
/// assert_eq!(Some(2), Week::Day("saturday".into()).tuple_count());
/// ```
pub trait TupleCount {
    fn tuple_count(&self) -> Option<usize>;
}

/// A trait that allows the modification of arguments within a tuple or struct variant.
/// Use [`delve_derive::EnumModify`] to automatically derive this trait.
///
/// # Example
///
/// ```rust
/// use delve::{EnumModify, ModifyField};
///
/// #[derive(EnumTuples)]
/// enum Thing {
///     Object {
///         x: usize,
///         y: usize,
///     }
/// }
///
/// let obj = Thing::Object { x: 0, y: 0 }
///
/// assert_eq!(Some(&0), obh.get_field("x"));
/// obj.set_field("x", 100);
/// assert_eq!(Some(&100), obh.get_field("x"));
/// ```
///
/// # Note
///
/// Generics and lifetimes will cause conflicting implementations if used within a tuple / struct variant that is not skipped. The `#[delve(skip)]`
/// attribute can be applied inside of the tuple / struct variant so the type is skipped:
///
/// ```rust
/// #[derive(EnumTuples)]
/// enum Week<'w, W> {
///     Day(#[delve(skip)] &'w String)
///     Other {
///         #[delve(skip)]
///         name: W,
///     }
/// }
/// ```
pub trait ModifyField<F, V> {
    fn get_field(&self, field: F) -> Option<&V>;
    fn set_field(&mut self, field: F, value: V) -> Option<()>;
}
