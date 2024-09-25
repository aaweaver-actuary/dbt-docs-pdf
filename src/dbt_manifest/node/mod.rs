pub mod node_fqn;
pub mod node_metadata;
pub mod node_config;

pub use node_fqn::NodeFqn;
pub use node_metadata::NodeMetadata;
pub use node_config::NodeConfig;

use crate::types::DbtChecksum;
use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct Node {
    pub name: String,
    pub fqn: NodeFqn,
    pub metadata: NodeMetadata,
    pub checksum: DbtChecksum,
    pub config: NodeConfig,
}
