 use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reuse ResourceType from existing Node implementation for node type
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphNode {
    pub name: String,           // Fully qualified name of the node
    pub node_type: ResourceType, // Reuse the existing ResourceType enum
    #[serde(default)]
    pub succ: Option<Vec<u32>>, // Optional successors
}

/// Represents the entire graph summary, where the key is the index and the value is the GraphNode
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphSummary {
    pub linked: HashMap<u32, GraphNode>, // The index is the key, and the node is the value
}