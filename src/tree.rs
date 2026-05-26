use std::collections::HashMap;
use crate::template::{Template, TokenSlot};
use crate::snapshot::DrainSnapshot;
use crate::tokenizer::{preprocess, tokenize};

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
                let rematch = template.try_match(tokens);
                MatchOutcome {
                    id: template.id(),
                    params: rematch.params.into_iter().map(|s| s.into()).collect(),
                    created: false,
                }
            }
            _ => {
                // ON miss create new template
                let new_template = Template::from_tokens(next_id_snapshot, tokens);
                let id = new_template.id();
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

    // **** Serialization Methods **** //
    
    /// Walk the learned tree and collect every templates
    /// into a flat snapshot
    pub fn dump(&self) -> DrainSnapshot {
        let mut templates = Vec::new();
        for node in self.by_length.values() {
            Self::collect_templates(node, &mut templates)
        }

        DrainSnapshot {
            version: DrainSnapshot::CURRENT_VERSION,
            threshold: self.threshold,
            next_id: self.next_id,
            templates: templates,
        }
    }

    // Rebuild method
    /// Rebuild a tree from a snapshot
    /// !! Each template is placed at:
    /// (slots.len(), first_literal_token)
    pub fn load(snapshot: DrainSnapshot) -> Self {
        let mut tree = Tree {
           by_length: HashMap::new(),
           next_id: snapshot.next_id,
           threshold: snapshot.threshold,
        };

        for template in snapshot.templates {
            let length = template.slots().len();
            let first_key = Self::first_token_key(template.slots());

            let leaf = tree.find_or_create_leaf_mut(length, &first_key);
            leaf.push(template);
        }

        tree
    }

    /// Helper to walk a treeNode
    fn collect_templates(node: &TreeNode, temp_out: &mut Vec<Template>) {
    match node {
        TreeNode::Length(map) => {
            for child in map.values() {Self::collect_templates(child, temp_out);}
        },
        TreeNode::Leaf(templates) => {
            for t in templates {temp_out.push(t.clone());}
        }
    }
    }

    /// Picking path key
    fn first_token_key(slot: &[TokenSlot]) -> Box<str> {
        match slot.first() {
            Some(TokenSlot::Literal(lit)) => lit.clone(),
            Some(TokenSlot::Wildcard) => "<*>".into(),
            None => unreachable!("NO tokenslot found"),
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

    #[test]
    fn identical_line_matches_existing_template() {
        let mut tree = Tree::new(0.5);
        let tokens = ["sshd[<*>]:", "Failed", "password"];
        let first = tree.match_or_insert(&tokens);
        let second = tree.match_or_insert(&tokens);
        assert_eq!(first.id, second.id);
        assert!(!second.created);
    }

    #[test]
    fn different_length_creates_separate_template() {
        let mut tree = Tree::new(0.5);
        let a = tree.match_or_insert(&["foo", "bar"]);
        let b = tree.match_or_insert(&["foo", "bar", "baz"]);
        assert_ne!(a.id, b.id);
        assert!(b.created);
    }

    #[test]
    fn different_first_token_creates_separate_template() {
        let mut tree = Tree::new(0.5);
        let a = tree.match_or_insert(&["sshd", "Failed", "password"]);
        let b = tree.match_or_insert(&["kernel", "Failed", "password"]);
        assert_ne!(a.id, b.id);
        assert!(b.created);
    }

    #[test]
    fn similar_line_merges_and_extracts_params() {
        let mut tree = Tree::new(0.5);
        let _ = tree.match_or_insert(&["Failed", "password", "for", "alice"]);
        let second = tree.match_or_insert(&["Failed", "password", "for", "bob"]);
        // Same template id — they merged
        assert_eq!(second.id, 0);
        assert!(!second.created);
        // The diverging position was promoted to wildcard, "bob" extracted
        assert_eq!(second.params, vec!["bob"]);
    }

    // Final load/dump roundtrip test
    #[test]
    fn roundtrip_dump_load() {
        let lines = [
            "sshd[1234]: Failed password for alice from 192.168.1.1",
            "sshd[5678]: Failed password for bob from 10.0.0.1",
            "sshd[9012]: Accepted password for carol from 172.16.0.1",
            "kernel: CPU0: temperature above threshold",
            "cron[111]: starting job at 0xdeadbeef",
            "sshd[2222]: Failed password for dave from 192.168.1.2",
        ];

        let mut temp1 = Tree::new(0.5);
        let ids_first : Vec<u64> = lines.iter().map(|l| {
            let pre = preprocess(l);
            let tokens = tokenize(&pre);
            temp1.match_or_insert(&tokens).id
        }).collect();


        // Round trip with json
        let snap = temp1.dump();
        let json = serde_json::to_string(&snap).expect("serialize");
        let snap2: crate::snapshot::DrainSnapshot =
            serde_json::from_str(&json).expect("deserialize");
        let mut t2 = Tree::load(snap2);

        // Second pass: same lines must produce same ids
        let ids_second: Vec<u64> = lines.iter().map(|l| {
            let pre = preprocess(l);
            let tokens = tokenize(&pre);
            t2.match_or_insert(&tokens).id
        }).collect();

        assert_eq!(ids_first, ids_second,
        "template ids diverged");
    }
}
