//! # NZFCC Library
//!
//! This library provides types and utilities for working with the New Zealand Financial Category Codes (NZFCC) system.
//!
//! [https://nzfcc.org/explore/](https://nzfcc.org/explore/)

include!(concat!(env!("OUT_DIR"), "/category_groups.rs"));
include!(concat!(env!("OUT_DIR"), "/nzfcc_codes.rs"));
