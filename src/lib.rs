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
pub mod snapshot;

use std::ffi::{c_char, c_int};


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
    tree: std::sync::Mutex<crate::tree::Tree>,
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
#[unsafe(no_mangle)]
pub extern "C" fn drain_parse(
    handle: *mut Drain,
    raw_line: *const c_char,
    template_id_out: *mut u64,
    params_out: *mut *mut *mut c_char,
    params_len: *mut c_int,
) -> c_int {
    if handle.is_null()
        || raw_line.is_null()
        || template_id_out.is_null()
        || params_out.is_null()
        || params_len.is_null()
    {
        return -1;
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(raw_line) };
    let Ok(raw) = c_str.to_str() else { return -1; };

    let drain = unsafe { &*handle };
    let mut tree = match drain.tree.lock() {
        Ok(g) => g,
        Err(_) => return -1,  // poisoned mutex
    };

    let preprocessed = crate::tokenizer::preprocess(raw);
    let tokens = crate::tokenizer::tokenize(&preprocessed);
    let outcome = tree.match_or_insert(&tokens);

    // Generate output
    unsafe {
        *template_id_out = outcome.id;
        *params_len = outcome.params.len() as c_int;

        if outcome.params.is_empty() {
            *params_out = std::ptr::null_mut();
        } else {
            let arr = libc::malloc(
                outcome.params.len() * std::mem::size_of::<*mut c_char>()
            ) as *mut *mut c_char;
            if arr.is_null() { return -1; }
            for (i, p) in outcome.params.iter().enumerate() {
                let cs = std::ffi::CString::new(p.as_str()).unwrap_or_default();
                *arr.add(i) = cs.into_raw();
            }
            *params_out = arr;
        }
    }
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn drain_create(threshold: f64) -> *mut Drain {
    let drain = Box::new(Drain {
        tree: std::sync::Mutex::new(crate::tree::Tree::new(threshold)),
        config: DrainConfig::default(),
    });
    Box::into_raw(drain)
}


#[unsafe(no_mangle)]
pub extern "C" fn drain_destroy(handle: *mut Drain) {
    if handle.is_null() { return; }
    unsafe { drop(Box::from_raw(handle)); }
}

#[unsafe(no_mangle)]
pub extern "C" fn drain_free_params(params: *mut *mut c_char, len: c_int) {
    if params.is_null() { return; }
    unsafe {
        for i in 0..len as usize {
            let ptr = *params.add(i);
            if !ptr.is_null() {
                drop(std::ffi::CString::from_raw(ptr));
            }
        }
        libc::free(params as *mut libc::c_void);
    }
}
