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

/// Preprocess Text
/// Match Regex with known patterns and replace with wildcard
/// Known patterns include:
/// - IP addresses
/// - HEX digits
/// - INT digits
///
/// # Arguments
/// 'raw' string of tokens to be preprocessed
/// # Returns
/// owned string with patterns replaced
pub fn preprocess(raw: &str) -> String {
    let cleaned_string = RE_IP.replace_all(raw, WILDCARD);
    let cleaned_string = RE_HEX.replace_all(&cleaned_string, WILDCARD);
    let cleaned_string = RE_INT.replace_all(&cleaned_string, WILDCARD);
    cleaned_string.into_owned()
} 

/// Tokenize
/// Strip whitespace and return a vector of tokens
pub fn tokenize(line: &str) -> Vec<&str> {line.split_whitespace().collect()}


pub fn tokenize_line(raw: &str) -> Vec<Box<str>> {
    let processed = preprocess(raw);
    tokenize(&processed)
        .into_iter()
        .map(|t| t.into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
     #[test]
    fn preprocess_replaces_ip() {
        assert_eq!(preprocess("connect from 192.168.1.1"), "connect from <*>");
    }

    #[test]
    fn preprocess_replaces_hex() {
        assert_eq!(preprocess("address 0xdeadbeef"), "address <*>");
    }

    #[test]
    fn preprocess_replaces_int() {
        assert_eq!(preprocess("pid 1234"), "pid <*>");
    }

    #[test]
    fn preprocess_replaces_pid_in_brackets() {
        assert_eq!(preprocess("sshd[1234]:"), "sshd[<*>]:");
    }
}
