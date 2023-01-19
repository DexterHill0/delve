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
