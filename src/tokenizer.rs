// Removing already known patterns by placeholders,
// conserves tree entropy, since every known will 
// eventually be crating a new branch, remove known
// placeholders with a placeholder
// A wildcard placeholder for repeated entries
// PIDS
// IPs

use std::sync::LazyLock;
use regex::Regex;

let RE_IP = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
let RE_HEX = Regex::new(r"\b0x[0-9a-fA-F]+\b").unwrap();
let RE_INT = Regex::new(r"\b\d+\b").unwrap();

const WILDCARD: &str = "<*>";

/// Preprocess
pub fn preprocess(raw: &str) -> String {todo!()} 

pub fn tokenize(line: &str) -> Vec<&str> {todo!()}
