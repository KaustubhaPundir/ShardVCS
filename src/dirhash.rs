use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn hash_dir(path: &Path) -> String {
    let mut entries = match fs::read_dir(path) {
        Ok(e) => e.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(_) => return String::new(),
    };

    entries.sort_by_key(|e| e.path());

    let mut combined = Vec::new();

    for entry in entries {
        let p = entry.path();

        if p.ends_with(".vcs") {
            continue;
        }

        if p.is_file() {
            if let Ok(data) = fs::read(&p) {
                combined.extend(data);
            }
        } else if p.is_dir() {
            combined.extend(hash_dir(&p).as_bytes());
        }
    }

    hash_bytes(&combined)
}

pub fn save(path: &Path, hash: &str) {
    let rel = path.strip_prefix(".").unwrap_or(path);

    let name = if rel.as_os_str().is_empty() {
        "root".to_string()
    } else {
        rel.to_string_lossy().replace("/", "_")
    };

    fs::create_dir_all(".vcs/dirhash").unwrap();

    fs::write(format!(".vcs/dirhash/{}", name), hash).unwrap();
}
