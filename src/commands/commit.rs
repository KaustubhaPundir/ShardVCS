use crate::{repo, index, object};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    timestamp: String,
    parent: Option<String>,
    files: Vec<(String, String)>,
}

pub fn run(message: &str) {
    repo::ensure_repo();

    let files: Vec<(String, String)> = index::load_all().into_iter().collect();

    if files.is_empty() {
        println!("nothing to commit");
        return;
    }

    let parent = fs::read_to_string(".vcs/HEAD")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let commit = Commit {
        message: message.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        parent,
        files,
    };

    let data = bincode::serialize(&commit).unwrap();
    let hash = object::store(&data);

    fs::write(".vcs/HEAD", &hash).unwrap();

    index::clear();

    println!("Committed {}", hash);
}
