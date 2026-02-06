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

pub fn run() {
    let mut head = fs::read_to_string(".vcs/HEAD").unwrap();
    while !head.is_empty() {
        let data = object::load(&head);
        let commit: Commit = bincode::deserialize(&data).unwrap();

        println!("commit {}", head);
        println!("Date: {}", commit.timestamp);
        println!("    {}\n", commit.message);

        head = commit.parent.unwrap_or_default();
    }
}
