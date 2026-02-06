use crate::repo;

pub fn run() {
    repo::init_repo();
    println!("Initialized empty VCS repository");
}
