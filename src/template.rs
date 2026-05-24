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
    /// Create a new tempalte entry from sequence of tokens
    ///
    /// Every Token starts as TokenSlot::literal<token>
    /// Wildcards are introduced on merge
    ///
    /// # Arguments
    /// * 'id' - The global template ID, immutable for life time
    /// * 'token' - Series of tokens extracted from log input
    pub fn new_template(id: TemplateId, token: &[&str])-> Self{
        let slots = token.iter()
        .copied()
        .map(|token| TokenSlot::Literal(token.into()))
        .collect();

        Self {
            id,
            slots,
            match_count: 0,
        }
    }

    /// Evaluate incoming tokens with an existing template,
    /// count matches and similarities. Extract wildcards.
    ///
    /// # Arguments
    /// *tokens* - the tokens being evaluated against
    ///
    /// # Return
    ///
    /// ['MatchResult'] with
    /// - similarity - ratio of match hits to total slots
    /// - prams - a vector of wildcard hits
    pub fn try_match(&self, tokens: &[&str])-> MatchResult{
        let mut matches = 0usize;
        let mut params: Vec<Box<str>> = Vec::new(); // The params that are signed as wildcard

        for (slot, token) in self.slots().iter().zip(tokens.iter().copied()) {
            match slot {
                TokenSlot::Literal(lit) => {
                    if &**lit == token {
                        matches += 1; // If token matches as a template literal
                                      // Count as a match
                    }
                }
                TokenSlot::Wildcard => {
                    // Wildcard matches also count according to drain
                    // spec.
                    matches += 1; 
                    params.push(token.into());
                }
            }
        }

        let similarity = matches as f64 / self.slots().len() as f64;
        MatchResult {similarity, params}
    }


    /// Promote diverging literal positions to wildcard
    ///
    /// # Arguments
    /// 'tokens' - tokens being evaluated against a template
    ///
    /// # Return 
    /// 'promoted' the number of literals promoted to wildcards
    pub fn merge(&mut self, tokens: &[&str])-> usize{
        let mut promoted = 0usize;

        for (slot, token) in self.slots.iter_mut().zip(tokens.iter().copied()) {
            match slot {
                TokenSlot::Literal(lit) => {
                    if lit.as_ref() != token {
                        *slot = TokenSlot::Wildcard;
                        promoted += 1;
                    }
                }
                TokenSlot::Wildcard => {}
            }
        }

        promoted
    }

    /// Bump match_count
    /// Called by the tree leaf when a match is won
    pub fn record_match(&mut self){ self.match_count += 1;}


    /// Constuct a template from tokens, i use it for testing other modules
    pub fn from_tokens(id: u64, tokens: &[&str]) -> Self {
        let slots = tokens
            .iter()
            .map(|t| TokenSlot::Literal((*t).into()))
            .collect();
        Self {
            id,
            slots,
            match_count: 1,
        }
    }
    /// Accessors
    pub fn id(&self) -> TemplateId {self.id as u64}
    pub fn len(&self) -> usize{self.slots.len()}
    pub fn match_count(&self)-> u64{self.match_count}
    pub fn slots(&self) -> &[TokenSlot]{&self.slots}

    /// Constuctor for tests
    #[cfg(test)]
    pub(crate) fn from_slots(id: TemplateId, slots: Vec<TokenSlot>) -> Self {
        Self { id, slots, match_count: 0 }
    }
}

impl std::fmt::Display for Template {
    // renders as "sshd Failed password <*>"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{todo!()}
}


#[cfg(test)]
mod tests {
    use super::*;
   // Identical literals are a match, try with template
   #[test]
   fn identical_tokens_score_one() {
        let new_template = Template::new_template(1, &["sshd", "Failed", "Pass"]);

        let result = new_template.try_match(&["sshd", "Failed", "Pass"]);
        assert_eq!(result.similarity, 1.0);
        assert!(result.params.is_empty());
        }

   // Try one template against mismatching literals
   #[test]
   fn mismatching_literals_with_similar_template() {
       let new_template = Template::new_template(1, &["sshd", "Failed", "Pass", "ROB"]);

       let result = new_template.try_match(&["sshd", "Nutz", "Pass", "ROB"]);
       assert!((result.similarity - 0.75).abs() < 1e-9);
       assert!(result.params.is_empty());
   }

   // Partial match with fixed ratio
   // ['a', 'b', 'c'] vs ['a', 'c', 'd'] - 0.75 silimlarity
   #[test]
   fn complete_mismatch_no_wildcard() {
       let new_template = Template::new_template(1, &["sshd", "Failed", "Pass", "ROB"]);

       let result = new_template.try_match(&["cron", "denial", "fix", "back"]);
       assert_eq!(result.similarity, 0.0);
       assert!(result.params.is_empty());
   }
   // Wildcard counts as a match - drain spec
   #[test]
   fn wildcard_counts_as_match() {
        let slots = vec![
            TokenSlot::Literal("sshd".into()),
            TokenSlot::Literal("pass".into()),
            TokenSlot::Literal("for".into()),
            TokenSlot::Wildcard,
        ];

        let new_template = Template::from_slots(1, slots);
        let result = new_template.try_match(&["sshd", "pass", "for", "alice"]);
        assert_eq!(result.similarity, 1.0);
        assert_eq!(result.params.len(), 1);
        assert_eq!(&*result.params[0], "alice");
   }
   // Merge promotes diverging point to a wildcard
   #[test]
   fn merge_promotes_diverging_points() {
       let mut temp = Template::new_template(1, &["Failed", "Pass", "For", "alice"]);

       let num_promoted = temp.merge(&["Failed", "Pass", "For", "bob"]);
       // BOB/ALICE - 4th token is diverging point
       assert_eq!(num_promoted, 1);
       assert_eq!(temp.slots(), &[
           TokenSlot::Literal("Failed".into()),
           TokenSlot::Literal("Pass".into()),
           TokenSlot::Literal("For".into()),
           TokenSlot::Wildcard,]);
   }
   // merging into existing wildcard should be a no-op
   #[test]
   fn mergeing_wildcards_shouldnt_be_op() {
        let slots = vec![
            TokenSlot::Literal("Failed".into()),
            TokenSlot::Literal("Pass".into()),
            TokenSlot::Literal("For".into()),
            TokenSlot::Wildcard,
        ];

        let mut temp = Template::from_slots(1, slots);
       let num_promoted = temp.merge(&["Failed", "Pass", "For", "bob"]);
       assert_eq!(num_promoted, 0);
       assert_eq!(temp.slots(), &[
           TokenSlot::Literal("Failed".into()),
           TokenSlot::Literal("Pass".into()),
           TokenSlot::Literal("For".into()),
           TokenSlot::Wildcard,]);

   }

   #[test]
   fn merge_promotes_multiple_diverging_positions() {
       let mut t = Template::new_template(1, &["a", "b", "c", "d", "e"]);

       let promoted = t.merge(&["a", "x", "c", "y", "z"]);

       assert_eq!(promoted, 3);
       assert_eq!(t.slots(), &[
           TokenSlot::Literal("a".into()),
           TokenSlot::Wildcard,
           TokenSlot::Literal("c".into()),
           TokenSlot::Wildcard,
           TokenSlot::Wildcard,
       ]);
   }

   #[test]
   fn record_match_increments_counter() {
       let mut t = Template::new_template(1, &["foo"]);

       assert_eq!(t.match_count(), 0);
       t.record_match();
       assert_eq!(t.match_count(), 1);
       t.record_match();
       assert_eq!(t.match_count(), 2);
   }
   // Count of merge is accurate? - on multiple merges
}
