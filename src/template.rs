/// What does a template hold
/// slots for templates
/// template ids
/// mathch count
///
/// Operations:
/// Create nodes form tokens
/// Produce similarity score for incoming tokens
/// merge similar tokens 
///

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSlot {
    /// A fixed token that must match exactly
    Literal(Box<str>),
    /// Matches any single token
    Wildcard,
}

/// Global template IDs
pub type TempalteId = u64;

/// Structural pattern that is extracted from log lines
#[derive(Debug)]
pub enum Template {
    id: TemplateId,
    slots: Vector<TokenSlot>,
    match_count: u64,
}

#[derive(Debug)]
pub struct MatchResult {
    /// similarity index between [0.1 - 1.0], wildcards also count as similarity
    pub similarity: f64,
    /// Tokens that landed on wildcard slots
    pub params: Vec<Box<str>>,
}


impl Template {

    /// Create a new tempalte entry, with every token starting
    /// as a literal
    pub fn new_tempalte(id: TemplateId, token: &[&str])-> Self;

    /// Score incoming tokens and extract params in one pass
    pub fn try_match(&self, tokens: &[&str])-> MatchResult;

    /// Promote diverging literal positions to wildcard
    /// Returns the number of slots newly promoted
    pub fn merge(&mut self, tokens: &str)-> usize;

    /// Bump match_count
    pub fn record_match(&mut self);


    /// Accessors
    pub fn id(&self) -> TemplateId;
    pub fn len(&self) -> usize;
    pub fn match_count(&self)-> u64;
    pub fn slote(&self) -> &[TokenSlot];
}

impl std::fmt::Display for Template {
    // renders as "sshd Failed password <*>"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
