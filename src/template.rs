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
pub type TemplateId = u64;

/// Structural pattern that is extracted from log lines
#[derive(Debug)]
pub struct Template {
    id: TemplateId,
    slots: Vec<TokenSlot>,
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
    pub fn new_template(id: TemplateId, token: &[&str])-> Self{todo!()}

    /// Score incoming tokens and extract params in one pass
    pub fn try_match(&self, tokens: &[&str])-> MatchResult{todo!()}


    /// Promote diverging literal positions to wildcard
    /// Returns the number of slots newly promoted
    pub fn merge(&mut self, tokens: &[&str])-> usize{todo!()}

    /// Bump match_count
    pub fn record_match(&mut self){todo!()}


    /// Accessors
    pub fn id(&self) -> TemplateId {todo!()}
    pub fn len(&self) -> usize{todo!()}
    pub fn match_count(&self)-> u64{todo!()}
    pub fn slote(&self) -> &[TokenSlot]{todo!()}
}

impl std::fmt::Display for Template {
    // renders as "sshd Failed password <*>"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{todo!()}
}


#[cfg(test)]
mod tests {
    use super::*;
   // Identical literals are a match, try with template
   fn identical_tokens_score_one() {
        let new_template = Template::new_template(1, &["sshd", "Failed", "Pass"]);

        let result = new_template.try_match(&["sshd", "Failed", "Pass"]);
        assert_eq!(result.similarity, 1.0);
        assert!(result.params.is_empty());
        }

   // Try one template against mismatching literals
   fn mismatching_literals_with_similar_template() {
       let new_template = Template::new_template(1, &["sshd", "Failed", "Pass", "ROB"]);

       let result = new_template.try_match(&["sshd", "Nutz", "Pass", "ROB"]);
       assert_eq!(result.similarity, 0.75);
       assert!(!(result.params.is_empty()));
   }
   //
   // Partial match with fixed ratio
   // ['a', 'b', 'c'] vs ['a', 'c', 'd'] - 0.75 silimlarity
   //
   // Wildcard counts as a match - drain spec
   //
   // Merge promotes diverging point to a wildcard
   // o
   // merging into existing wildcard should be a no-op
   //
   // Count of merge is accurate? - on multiple merges
   //
   //
}
