use std::io::{self, BufRead};
use dRain::tree::Tree;
use dRain::tokenizer::{preprocess, tokenize_line};

fn main() {
    let mut tree = Tree::new(0.5);
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let raw = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if raw.is_empty() { continue; }

        let tokens = tokenize_line(&raw);  
        let refs: Vec<&str> = tokens.iter().map(|s| s.as_ref()).collect();
        let outcome = tree.match_or_insert(&refs);

        print!("{}", outcome.id);
        for p in &outcome.params {
            print!("\t{}", p);
        }
        println!();
    }
}


