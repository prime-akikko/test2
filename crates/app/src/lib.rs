//! GPUI starter application library.
//!
//! - Always available: German strings + thin shell state helpers.
//! - Feature `gui`: GPUI window bootstrap ([`ui`]).

#![forbid(unsafe_code)]

pub mod strings;

#[cfg(feature = "gui")]
pub mod ui;
