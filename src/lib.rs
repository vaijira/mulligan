#![warn(rust_2018_idioms, missing_docs, warnings)]

//! Library to simulate, collect and organize finance information.
//!
//! Current support:
//! * Extract H.4.1 federal reserve balance sheet information.

#[macro_use]
extern crate lazy_static;

/// Provides parsing functionality to extract federal reserve information.
pub mod fed;
mod iter;
mod types;

pub use self::types::ASSETS_PATH;
pub use self::types::{BalanceSheet, Concept, ConceptType};
pub use chrono::NaiveDate;
