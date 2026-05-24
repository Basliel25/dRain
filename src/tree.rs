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
}


