use sha2::{Sha256,Digest};
use std::fs;
use std::path::Path;
use crate::index::node::{IndexNode, NodeType};

pub fn compute_hash(node: &mut IndexNode, base: &Path) -> String {
    let mut hasher = Sha256::new();

    match node.node_type {
        NodeType::File => {
            let content = fs::read(base.join(&node.name)).unwrap();
            hasher.update(content);
        }
        NodeType::Directory => {
            for child in node.children.values_mut() {
                let child_hash = compute_hash(child, &base.join(&node.name));
                hasher.update(child_hash);
            }
        }
    }

    let result = format!("{:x}", hasher.finalize());
    node.hash=Some(result.clone());
    result
}