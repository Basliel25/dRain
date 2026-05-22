/// Removing already known patterns by placeholders,
/// conserves tree entropy, since every known will 
/// eventually be crating a new branch, remove known
/// placeholders with a placeholder
/// A wildcard placeholder for repeated entries
/// PIDS
/// IPs
///
///This is going to be done using regex matching
const WILDCARD: &str = "<*>";

/// Preprocess
pub fn preprocess(raw: &str) -> String {todo!()} 

pub fn tokenize(line: &str) -> Vec<&str> {todo!()}
