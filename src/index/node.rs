use std::collections::HashMap;

#[derive(Debug)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct IndexNode {
    pub name: String,
    pub node_type: NodeType,
    pub size: u64,
    pub mtime: u64,
    pub dirty: bool,
    pub children: HashMap<String, IndexNode>,
}