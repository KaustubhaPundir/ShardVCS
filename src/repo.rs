use std::fs;
use std::path::Path;

pub fn ensure_repo() {
    if !Path::new(".vcs").exists() {
        panic!("Not a VCS repository");
    }
}

pub fn init_repo() {
    fs::create_dir(".vcs").unwrap();
    fs::create_dir(".vcs/objects").unwrap();
    fs::create_dir(".vcs/index").unwrap();
    fs::write(".vcs/HEAD", b"").unwrap();
}
