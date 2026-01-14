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
//!
//! // Convert to and from strings
//! let group_str = CategoryGroup::ProfessionalServices.to_string();
//! assert_eq!(group_str, "Professional Services");
//!
//! let parsed_group: CategoryGroup = "Professional Services".parse().unwrap();
//! assert_eq!(parsed_group, CategoryGroup::ProfessionalServices);
//!
//! // Use TryFrom for conversion
//! let group = CategoryGroup::try_from("Household").unwrap();
//! assert_eq!(group, CategoryGroup::Household);
//!
//! // Iterate over all possible values
//! let all_groups = CategoryGroup::values();
//! assert_eq!(all_groups.len(), 10);
//!
//! let all_codes = NzfccCode::values();
//! assert_eq!(all_codes.len(), 181);
//!
//! // Use AsRef<str> for ergonomic string conversion
//! fn print_group(group: impl AsRef<str>) {
//!     println!("{}", group.as_ref());
//! }
//! print_group(CategoryGroup::Health);
//!
//! // Convert to static string slice with From
//! let s: &'static str = CategoryGroup::Transport.into();
//! assert_eq!(s, "Transport");
//!
//! // Get variant name for debugging/serialization
//! let variant = CategoryGroup::Education.variant_name();
//! assert_eq!(variant, "Education");
//!
//! // Iterate over all values
//! for group in CategoryGroup::iter().take(3) {
//!     println!("{}", group);
//! }
//!
//! // Use in sorted collections (Ord is implemented)
//! use std::collections::BTreeSet;
//! let mut set = BTreeSet::new();
//! set.insert(CategoryGroup::Food);
//! set.insert(CategoryGroup::Housing);
//! ```
//!
//! ## Features
//!
//! - **`serde`**: Enable serialization/deserialization support
//! - **`arbitrary`**: Enable `arbitrary::Arbitrary` for fuzzing and property testing
//! - **`clap`**: Enable `clap::ValueEnum` for CLI argument parsing

// Include generated code
mod generated;

pub use generated::{CategoryGroup, NzfccCode, ParseCategoryGroupError, ParseNzfccCodeError};
