// src/commands/checkout.rs
use crate::object;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    timestamp: String,
    parent: Option<String>,
    files: Vec<(String, String)>,
}

pub fn run(hash: &str) {
    let data = object::load(hash);
    let commit: Commit = bincode::deserialize(&data).unwrap();

    for (path, file_hash) in commit.files {
        let contents = object::load(&file_hash);
        fs::write(path, contents).unwrap();
    }

    fs::write(".vcs/HEAD", hash).unwrap();
    println!("Checked out {}", hash);
}
