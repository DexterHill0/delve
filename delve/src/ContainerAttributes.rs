//! # Container Attributes
//!
//! - `#[delve(rename_all = "<inflection>")]`
//!
//!     Renames all identifiers, including all field names (inside struct variants). This attribue has
//!     the lowest priority meaning the other two attributes will overwrite their respective identifies,
//!     instead of this one.
//!     The inflection can be any one of:
//! - `uppercase`
//! - `lowercase`
//! - `camelcase`
//! - `snakecase`
//! - `pascalcase`
//! - `screamingsnakecase`
//! - `kebabcase`
//!     - `#[delve(rename_all = "snake_case")]`
//!     - `#[delve(rename_all = "SCREAMINGSNAKE")]`
//!
//!
//! - `#[delve(rename_fields = "<inflection>")]`
//!
//! Renames all fields within all struct variants.
//!
//! - `#[delve(rename_variants = "<inflection>")]`
//!
//! Renames all fields within all struct variants
//!
//! - `#[delve(resolve(<type>, <resolved>))]`
//!
//!     Only affects [`ModifyVariant`]. Resolves type aliased to a given type. If multiple aliases alias to the
//!     same type, they will be implemented as if they were different type. The `resolve` attribute can be used to
//!     stop that from happening.
//!     - `#[delve(resolve(Foo, u16), resolve(Bar, u16))]` - `Foo` and `Bar` will now be implemented as a single `u16`.
//!
