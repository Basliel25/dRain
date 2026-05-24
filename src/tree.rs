use std::collections::HashMap;
use crate::template::Template;

pub struct Tree {
    by_length: HashMap<usize, TreeNode>,
    next_id: u64,
    threshold: f64,
}

enum TreeNode {
    Length(HashMap<Box<str>, TreeNode>),
    Leaf(Vec<Template>),
}

pub struct MatchOutcome {
    pub id: u64,
    pub params: Vec<String>,
    pub created: bool,
}

impl Tree {
    pub fn new(threshold: f64) -> Self {
        Self {
            by_length: HashMap::new(),
            next_id: 0,
            threshold,
        }
    }
    pub fn match_or_insert(&mut self, tokens: &[&str]) -> MatchOutcome {todo!()}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_empty_tree() {
        let mut tree = Tree::new(0.5);
        let tokens = ["sshd[<*>]:", "Failed", "password"];
        let outcome = tree.match_or_insert(&tokens);
        assert_eq!(outcome.id, 0);
        assert!(outcome.created);
        assert!(outcome.params.is_empty());
    }
}
