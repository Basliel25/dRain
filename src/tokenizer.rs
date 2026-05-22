// Removing already known patterns by placeholders,
// conserves tree entropy, since every known will 
// eventually be crating a new branch, remove known
// placeholders with a placeholder
// A wildcard placeholder for repeated entries
// PIDS
// IPs

use std::sync::LazyLock;
use regex::Regex;

static RE_IP:  LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"
).unwrap());

static RE_HEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"\b0x[0-9a-fA-F]+\b"
).unwrap());

static RE_INT: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"\b\d+\b"
).unwrap());

const WILDCARD: &str = "<*>";

/// Preprocess
pub fn preprocess(raw: &str) -> String {
    let cleaned_string = RE_IP.replace_all(raw, WILDCARD);
    let cleaned_string = RE_HEX.replace_all(&cleaned_string, WILDCARD);
    let cleaned_string = RE_INT.replace_all(&cleaned_string, WILDCARD);
    cleaned_string.into_owned()
} 

pub fn tokenize(line: &str) -> Vec<&str> {todo!()}
