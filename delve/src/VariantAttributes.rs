//! # Variant Attributes
//!
//! - `#[delve(skip)]`
//!
//! Skips the respective variant.
//!
//! - `#[delve(ascii_case_insensitive)]`
//!
//! Applies only to `EnumFromStr`. This attribute means the variant can be constructed
//! from a string of any case. Currently only ASCII is supported.
//!
//! - `#[delve(rename_variant = "<name>")]`
//!
//!     Renames the respective variant with the given name. This overides any container attributes
//!     if applied.
//!     - `#[delve(rename_variant = "foobar")]`
//!
//! - `#[delve(from = "<from>")]`
//!
//!     Applies only to `EnumFromStr`. This attribute changes what the string that is looked
//!     for in `FromStr`. Multiple `from`'s can be provided for multiple matches. This overides
//!     any container attributes if applied.
//!     - `#[delve(from = "foo")]`
//!     - `#[delve(from = "foo", from = "bar", ...)]`
//!
//! - `#[delve(to = "<to>")]`
//!
//!     Applies only to `EnumToStr`. This attribute changes what string the enum variant is
//!     converted to. This overides any container attributes if applied.
//!     - `#[delve(to = "foo")]`
//!
//! - `#[delve(display = <fn>/<closure>/"<dsiplay>")]`
//!
//!     Applies only to `EnumDisplau`. This attribute changes what string is display when using
//!     the Display formatter. The attribute can take one of three types: a function name, a closure,
//!     or a string. If given a closure or function name, the arguments of the function will match
//!     those of the values in the respective variant (if any).
//!     - `#[delve(display = "foo")]`
//!     - `#[delve(display = || "foo")]`
//!     - `#[delve(display = |a1: ..., ...| "foo")]`
//!     - `#[delve(display = foo_fn)]`
//!
