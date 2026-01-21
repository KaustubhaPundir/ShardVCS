use std::env;
use std::fs;
mod index;
use std::path::Path;
use index::scanner::scan;
fn main(){
    // let args: Vec<String> = env::args().collect();

    // if(args.len() < 2){
    //     eprintln!("Usage: vcs <command>");
    //     return;
    // }

    // match args[1].as_str(){
    //     "init" => init_repo(),
    //     _ => eprintln!("unknown command"),
    // }
    let tree=scan(Path::new("."));
    println!("{:#?}",tree);
}

fn init_repo(){
    fs::create_dir(".vcs").expect("Failed to create .vcs");
    fs::create_dir(".vcs/index").unwrap();
    fs::create_dir(".vcs/objects").unwrap();
    println!("Initialized empty VCS repository");
}