//src/commands/add.rs
use std::fs;
use std::path::{Path, PathBuf};

use crate::{repo, object, index};

fn walk(root: &Path, path: &Path) {
    if path.ends_with(".vcs") {
        return;
    }

    if path.is_dir() {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries {
            if let Ok(entry) = entry {
                walk(root, &entry.path());
            }
        }
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
    }
}

pub fn run(input: &str) {
    repo::ensure_repo();

    let root = PathBuf::from(".");
    let target = PathBuf::from(input);

    walk(&root, &target);
}
