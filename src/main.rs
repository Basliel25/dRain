use std::env;
use std::io::{self, BufRead, Write};
use std::fs;
use dRain::tree::Tree;
use dRain::tokenizer::{preprocess, tokenize_line, tokenize};
use dRain::snapshot::DrainSnapshot;
use dRain::logFormat::LogFormat;

fn main()-> std::process::ExitCode {

    let args: Vec<String> = env::args().collect();
    let mut load_path: Option<String> = None;
    let mut save_path: Option<String> = None;

    let mut i = 1;
    let mut log_format = LogFormat::PassThrough;

    while i < args.len() {
        match args[i].as_str(){
            "--load" | "-l" => {
                let Some(p) = args.get(i+1) else {
                    eprintln!("error: load requires a path");
                    return std::process::ExitCode::from(2);
                };
                load_path = Some(p.clone());
                i += 2;
            },

            "--save" | "-s" => {
                let Some(p) = args.get(i+1) else {
                    eprintln!("error: save requires a path");
                    return std::process::ExitCode::from(2);
                };
                save_path = Some(p.clone());
                i += 2;
            },
            "-h" | "--help" => {
                eprintln!("Usage: drain [--load <path>] | [--save <path>]");
                return std::process::ExitCode::SUCCESS;
            }

            other => {
                eprintln!("Unknown argumet, use -h to see possible arguments");
                    return std::process::ExitCode::from(2);
            }
        }
    }

    // Build a tree from path or from scratch
    let mut tree = match load_path {
        Some(path) => {
            let json = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: Error reading file {} \n{}\n", path, e);
                    return std::process::ExitCode::from(1);
                }
            };

            let snap: DrainSnapshot = match serde_json::from_str(&json) {
                Ok(s) => s, 
                Err(e) => {
                    eprintln!("error: Error reading file {} \n{}\n", path, e);
                    return std::process::ExitCode::from(1);
                }
            };
            Tree::load(snap)
        },
        None => Tree::new(0.1)
    };
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Drain Straem
    for line in stdin.lock().lines() {
        let raw = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if raw.is_empty() { continue; }

        let pre = preprocess(&raw);
        let tokens = tokenize(&pre);
        let outcome = tree.match_or_insert(&tokens);

        let _ = write!(out, "{}", outcome.id);
        for p in &outcome.params {
            let _ = write!(out, "\t{}", p);
        }
        let _ = writeln!(out);
    }


    // Save if requested
    if let Some(path) = save_path {
        let snap = tree.dump();
        let json = match serde_json::to_string_pretty(&snap) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: failed to serialize tree: {}", e);
                return std::process::ExitCode::from(1);
            }
        };

        if let Err(e) = fs::write(&path, json) {
            eprintln!("error: Failed to write {}: {}", path, e);
            return std::process::ExitCode::from(1);
        }
    }

    std::process::ExitCode::SUCCESS
}


