<<<<<<< HEAD
use std::collections::HashMap;
=======
//src/commands/add.rs
>>>>>>> 915a457cc94e565af96a426a32315cabde362ea9
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

<<<<<<< HEAD
use crate::{index, object, repo};

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    timestamp: String,
    parent: Option<String>,
    files: Vec<(String, String)>,
}

fn load_tracked() -> HashMap<String, String> {
    let head = fs::read_to_string(".vcs/HEAD").ok();
    if head.is_none() {
        return HashMap::new();
    }

    let h = head.unwrap().trim().to_string();
    if h.is_empty() {
        return HashMap::new();
    }

    let data = object::load(&h);
    let commit: Commit = bincode::deserialize(&data).unwrap();
    commit.files.into_iter().collect()
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if dir.ends_with(".vcs") {
=======
fn walk(root: &Path, path: &Path) {
    if path.ends_with(".vcs") {
>>>>>>> 915a457cc94e565af96a426a32315cabde362ea9
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, files);
            } else if path.is_file() {
                files.push(path);
            }
        }
<<<<<<< HEAD
=======
    } else if path.is_file() {
        let data = fs::read(path).expect("failed to read file");
        let hash = object::store(&data);

        let relative = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        index::add(&relative, &hash);
        println!("added {}", relative);
>>>>>>> 915a457cc94e565af96a426a32315cabde362ea9
    }
}

pub fn run(path: &str) {
    repo::ensure_repo();

    let tracked = load_tracked();
    let mut files = Vec::new();

    let p = Path::new(path);
    if p.is_dir() {
        collect_files(p, &mut files);
    } else {
        files.push(p.to_path_buf());
    }

    for path in files {
        let rel = path
            .strip_prefix(".")
            .unwrap_or(&path)
            .to_string_lossy()
            .trim_start_matches('/')
            .to_string();

        let data = fs::read(&path).unwrap();

        // IMPORTANT: compare using the SAME hash used by store()
        let current = object::hash_bytes(&data);

        // If file is tracked and unchanged, skip
        if let Some(old) = tracked.get(&rel) {
            if old == &current {
                continue;
            }
        }

        // Only now store + stage
        let stored = object::store(&data);
        index::add(&rel, &stored);
        println!("added {}", rel);
    }
}
