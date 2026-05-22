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

// FFI Entry Points

/// C-compatible parse function for use with Trafilo.
/// # Arguments
/// * `raw_line`   - Null-terminated C string (owned by caller).
/// * `template_id_out` - Out parameter for the template ID.
/// * `params_out` - Out parameter for the parameter array (allocated here, caller frees).
/// * `params_len` - Out parameter for the number of parameters.
///
/// # Returns
/// * `0` on success.
/// * `-1` on failure (null pointer, allocation error).
#[no_mangle]
pub extern "C" fn drain_parse(
    raw_line: *const c_char,
    template_id_out: *mut u64,
    params_out: *mut *mut c_char,
    params_len: *mut c_int,
) -> c_int {
    if raw_line.is_null()
        || template_id_out.is_null()
        || params_out.is_null()
        || params_len.is_null()
    {
        return -1;
    }

    // Convert C string to Rust &str
    let c_str = unsafe { std::ffi::CStr::from_ptr(raw_line) };
    let Ok(raw) = c_str.to_str() else {
        return -1;
    };

    //   Takss
    //   Call Drain::parse(raw)
    //   Write template_id to template_id_out
    //   Allocate params array and copy parameter strings
    //   Write count to params_len

    unsafe {
        *template_id_out = 0;
        *params_len = 0;
        *params_out = std::ptr::null_mut();
    }
    0
}


