use std::fs;
use std::path::Path;
use crate::index::node::{IndexNode,NodeType};

pub fn scan(path: &Path) -> IndexNode{
    let metadata=fs::metadata(path).unwrap();

    if metadata.is_dir(){
        let mut node=IndexNode{
            name: path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            node_type:NodeType::Directory,
            size:0,
            mtime:metadata.modified().unwrap()
                .elapsed().unwrap().as_secs(),
            dirty:false,
            children: Default::default(),
        };

        for entry in fs::read_dir(path).unwrap(){
            let entry=entry.unwrap();
            let child=scan(&entry.path());
            node.children.insert(child.name.clone(),child);
        }

        node
    }
    else{
        IndexNode {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            node_type: NodeType::File,
            size: metadata.len(),
            mtime: metadata.modified().unwrap()
                .elapsed().unwrap().as_secs(),
            dirty: false,
            children: Default::default(),
        }
    }
}