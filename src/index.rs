use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexEntry {
    pub path: String,
    pub hash: String,
}
pub fn clear() {
    if let Ok(entries) = fs::read_dir(".vcs/index") {
        for entry in entries {
            if let Ok(entry) = entry {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}
fn shard_for_path(path: &str) -> String {
    path.split('/').next().unwrap_or("root").to_string()
}

pub fn add(path: &str, hash: &str) {
    let shard_name = shard_for_path(path);
    let idx_path = format!(".vcs/index/{}.idx", shard_name);

    // Load all entries
    let mut map = load_all();
    map.insert(path.to_string(), hash.to_string());

    // Only write entries belonging to this shard
    let entries: Vec<IndexEntry> = map.iter()
        .filter(|(p, _)| shard_for_path(p) == shard_name)
        .map(|(p, h)| IndexEntry {
            path: p.clone(),
            hash: h.clone(),
        })
        .collect();

    fs::write(idx_path, bincode::serialize(&entries).unwrap()).unwrap();
}

pub fn load_all() -> HashMap<String, String> {
    let mut map = HashMap::new();

    if let Ok(dir) = fs::read_dir(".vcs/index") {
        for entry in dir {
            let entry = entry.unwrap();
            let data = fs::read(entry.path()).unwrap();
            let entries: Vec<IndexEntry> = bincode::deserialize(&data).unwrap();
            for e in entries {
                map.insert(e.path, e.hash);
            }
        }
    }

    map
}
