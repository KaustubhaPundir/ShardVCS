use std::fs;
use std::path::Path;
use crate::index::node::{IndexNode, NodeType};

pub fn detect_changes(node: &mut IndexNode, base: &Path){
    let current = base.join(&node.name);

    match node.node_type{
        NodeType::File=>{
            if let Ok(meta)=fs::metadata(&current){
                if meta.len()!=node.size{
                    node.dirty=true;
                }
            }
        }
        NodeType::Directory=>{
            for child in node.children.values_mut(){
                detect_changes(child,&current);
                if child.dirty {
                    node.dirty=true;
                }
            }
        }
    }
}