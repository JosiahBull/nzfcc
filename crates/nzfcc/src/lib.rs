//! # NZFCC Library
//!
//! This library provides types and utilities for working with the New Zealand Financial Category Codes (NZFCC) system.
//!
//! [https://nzfcc.org/explore/](https://nzfcc.org/explore/)
//!
//! ## Usage
//!
//! The library provides two main enums:
//! - [`CategoryGroup`] - High-level groupings (e.g., "Professional Services", "Household")
//! - [`NzfccCode`] - Specific category codes (e.g., "Cafes and restaurants")
//!
//! ## Example
//!
//! ```rust
//! use nzfcc::{CategoryGroup, NzfccCode};
//!
//! // Access category group IDs
//! let id = CategoryGroup::ProfessionalServices.id();
//! assert!(id.starts_with("group_"));
//!
//! // Access NZFCC code IDs
//! let code_id = NzfccCode::CafesAndRestaurants.id();
//! assert!(code_id.starts_with("nzfcc_"));
//!
//! // Get the category group for an NZFCC code
//! let group = NzfccCode::CafesAndRestaurants.group();
//! assert_eq!(group, CategoryGroup::Lifestyle);
//!
//! // Get all codes in a category group
//! let lifestyle_codes = CategoryGroup::Lifestyle.codes();
//! assert!(lifestyle_codes.contains(&NzfccCode::CafesAndRestaurants));
//! assert!(lifestyle_codes.len() > 0);
//! ```

// Include generated code
mod generated;

pub use generated::{CategoryGroup, NzfccCode};
