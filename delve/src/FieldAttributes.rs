//! # Field Attributes
//!
//! - `#[delve(rename_field = "<name>")]`
//! 
//!   Renames a single field within a struct variant. This overides any container attributes if applied.
//!   - `#[delve(rename_field = "foobar")]`
//!
//! - `#[delve(skip)]`
//!
//! Skips the respective field.
