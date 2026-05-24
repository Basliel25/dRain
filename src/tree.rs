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

fn score_leaf(leaf: &[Template], 
    tokens: &[&str]
) -> Option<(usize, crate::template::MatchResult)> {

    //Return the best candidates index as a matchresult struct
    //from templates
    leaf.iter()
        .enumerate()
        .map(|(i, token)| (i, token.try_match(tokens)))
        .max_by(|a, b| {
            a.1.similarity.partial_cmp(&b.1.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

impl Tree {
    pub fn new(threshold: f64) -> Self {
        Self {
            by_length: HashMap::new(),
            next_id: 0,
            threshold,
        }
    }
    fn find_or_create_leaf_mut(&mut self, 
        length: usize, 
        first_token:&str) -> &mut Vec<Template> {

        // Translate the length of a node
        let length_node = self.by_length.entry(length)
            .or_insert_with(|| TreeNode::Length(HashMap::new()));

        // Create a branch based on token length
        let map = match length_node {
            TreeNode::Length(m) => m,
            TreeNode::Leaf(_) => unreachable!("by_legth or tracks length nodes"),
        };

        // Create a leaf for token templates
        let leaf_node = map.entry(first_token.into())
            .or_insert_with(|| TreeNode::Leaf(Vec::new()));

        match leaf_node {
            TreeNode::Leaf(v) => v, 
            TreeNode::Length(_) => unreachable!("second level only contains leaf"),
        }
    }
    pub fn match_or_insert(&mut self, tokens: &[&str]) -> MatchOutcome {
    // Guard against empty lines, create sentinerl and stop
    if tokens.is_empty() {
        return MatchOutcome {
            id: u64::MAX,
            params: Vec::new(),
            created: false,
        };
    }

    let length = tokens.len();
    let first_token = tokens[0];
    let threshold = self.threshold;

    // Pull next_id into a local copy avoid double-borrow with &mut.
    let next_id_snapshot = self.next_id;

    let leaf = self.find_or_create_leaf_mut(length, first_token);

    // Score match with existing templates
    let best = score_leaf(leaf, tokens);

    match best {
        Some((idx, result)) if result.similarity >= threshold => {
            // Match found merge tokens update count
            let template = &mut leaf[idx];
            template.merge(tokens);
            template.record_match();
            MatchOutcome {
                id: template.id,
                params: result.params,
                created: false,
            }
        }
        _ => {
            // ON miss create new template
            let new_template = Template::from_tokens(next_id_snapshot, tokens);
            let id = new_template.id;
            leaf.push(new_template);
            self.next_id += 1;
            MatchOutcome {
                id,
                params: Vec::new(),
                created: true,
            }
        }
    }
}
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
