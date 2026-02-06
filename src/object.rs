use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn store(data: &[u8]) -> String {
    let hash = hash_bytes(data);

    let (dir, file) = hash.split_at(2);
    let dir_path = format!(".vcs/objects/{}", dir);
    let file_path = format!("{}/{}", dir_path, file);

    if !Path::new(&dir_path).exists() {
        fs::create_dir(&dir_path).unwrap();
    }

    fs::write(file_path, data).unwrap();
    hash
}

pub fn load(hash: &str) -> Vec<u8> {
    let (dir, file) = hash.split_at(2);
    fs::read(format!(".vcs/objects/{}/{}", dir, file)).unwrap()
}
