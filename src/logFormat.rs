use std::sync::LazyLock;
use regex::Regex;

/// An enum representing all supported log format
/// Preproccess happens based on log format
/// If the format is not supported drain process
/// with default tokenization
#[derive(Debug, copy, Clone)]
pub enum LogFormat {
    Linux,
    PassThrough,
}


impl LogFormat {
    pub fn strip_preamble<'a>(&self, line: &'a str) -> Option<&'a str>{

        match self {
            LogFormat::Linux => extract_linux(line),
            LogFormat::PassThrough => Some(line),
        }
    }

    fn extract_linx(&'a str) -> Option<&'a str> {Some(line)}
}
