use crate::{repo, index, object};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Commit {
    message: String,
    timestamp: String,
    parent: Option<String>,
    files: Vec<(String, String)>,
}

fn load_tracked_from_head() -> HashMap<String, String> {
    let head = match fs::read_to_string(".vcs/HEAD") {
        Ok(h) => h.trim().to_string(),
        Err(_) => return HashMap::new(),
    };

    if head.is_empty() {
        return HashMap::new();
    }

    let data = object::load(&head);
    let commit: Commit = bincode::deserialize(&data).unwrap();

    commit.files.into_iter().collect()
}
//working tree collection
fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if dir.ends_with(".vcs") {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, files);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
}

pub fn run() {
    repo::ensure_repo();

    let staged = index::load_all();
    let tracked = load_tracked_from_head();
    let mut working_files = Vec::new();

    collect_files(Path::new("."), &mut working_files);

    let mut seen = HashSet::new();

    let mut staged_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut untracked_files = Vec::new();
    let mut deleted_files = Vec::new();

    for path in working_files {
        let rel = path
            .strip_prefix(".")
            .unwrap_or(&path)
            .to_string_lossy()
            .trim_start_matches('/')
            .to_string();

        seen.insert(rel.clone());

        if let Some(staged_hash) = staged.get(&rel) {
            let data = fs::read(&path).unwrap();
            let cur_hash = object::hash_bytes(&data);

            if &cur_hash == staged_hash {
                staged_files.push(rel);
            } else {
                modified_files.push(rel);
            }
        } else if let Some(tracked_hash) = tracked.get(&rel) {

            let data = fs::read(&path).unwrap();
            let cur_hash = object::hash_bytes(&data);

            if &cur_hash != tracked_hash {
                modified_files.push(rel);
            }
        } else {
            untracked_files.push(rel);
        }
    }
//deleted files
    for path in tracked.keys() {
        if !seen.contains(path) {
            deleted_files.push(path.clone());
        }
    }

    //output
    if staged_files.is_empty()
        && modified_files.is_empty()
        && deleted_files.is_empty()
        && untracked_files.is_empty()
    {
        println!("nothing to commit, working tree clean");
        return;
    }

    if !staged_files.is_empty() {
        println!("staged files:");
        for f in staged_files {
            println!("  {}", f);
        }
    }

    if !modified_files.is_empty() {
        println!("\nmodified files:");
        for f in modified_files {
            println!("  {}", f);
        }
    }

    if !deleted_files.is_empty() {
        println!("\ndeleted files:");
        for f in deleted_files {
            println!("  {}", f);
        }
    }

    if !untracked_files.is_empty() {
        println!("\nuntracked files:");
        for f in untracked_files {
            println!("  {}", f);
        }
    }
}
