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

// *** Regex patterns **//
static RE_LINUX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"^\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\s+\S+\s+\S+?:\s+(?P<content>.*)$"
).unwrap());


impl LogFormat {
    pub fn strip_preamble<'a>(&self, line: &'a str) -> Option<&'a str>{

        match self {
            LogFormat::Linux => extract_linux(line),
            LogFormat::PassThrough => Some(line),
        }
    }

    fn extract_linx(line: &str) -> Option<&str> {
        RE_LINX().captures(line)
            .and_then(|c| c.name("content"))
            .map(|m| m.as_str())
    }
}
