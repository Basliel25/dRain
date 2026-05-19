//! # drain-c
//!
//! Streaming log template extraction engine.
//!
//! Implements the Drain algorithm (He et al., ICWS 2017)
//! as a fixed-depth tree parser
//!
//! ## FFI
//!
//! This library exposes a C-compatible interface for use with
//! the [trafilo](https://github.com/Basliel25/trafilo) streaming framework.
//! The primary entry point is [`drain_parse`].

pub mod tokenizer;
pub mod tree;
pub mod template;

use std::ffi::{c_char, c_int};
use std::sync::RwLock;


/// Configuration for a Drain parser instance.
pub struct DrainConfig {
    /// Fixed tree depth (default would be 4 or 6)
    pub depth: usize,
    /// Similarity threshold for template merging (default: 0.5)
    pub similarity_threshold: f64,
    /// Maximum children per internal bucket
    pub max_children_per_bucket: usize,
    /// Maximum templates per leaf cluster
    pub max_templates_per_leaf: usize,
}

impl Default for DrainConfig {
    fn default() -> Self {
        Self {
            depth: 4,
            similarity_threshold: 0.5,
            max_children_per_bucket: 100,
            max_templates_per_leaf: 1000,
        }
    }
}

/// A Drain log template parser.
pub struct Drain {
    config: DrainConfig,
}


