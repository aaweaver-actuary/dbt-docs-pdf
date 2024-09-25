use crate::types::dbt_resource_type::DbtResourceType;
use derive_builder::Builder;

#[derive(Builder, Debug, Clone)]
pub struct NodeMetadata {
    pub unique_id: String,
    pub resource_type: DbtResourceType,
    pub dbt_package_name: String,
    pub model_path: String,
    pub original_file_path: String,
    pub alias: String,
    pub model_name: String,
}

impl NodeMetadata {
    pub fn new(
        unique_id: String,
        resource_type: DbtResourceType,
        dbt_package_name: String,
        model_path: String,
        original_file_path: String,
        alias: String,
        model_name: String,
    ) -> Self {
        NodeMetadata {
            unique_id,
            resource_type,
            dbt_package_name,
            model_path,
            original_file_path,
            alias,
            model_name,
        }
    }

    pub fn builder() -> NodeMetadataBuilder {
        NodeMetadataBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_metadata_builder() {
        let node_metadata = NodeMetadata::builder()
            .unique_id("unique_id".to_string())
            .resource_type(DbtResourceType::Model)
            .dbt_package_name("dbt_package_name".to_string())
            .model_path("model_path".to_string())
            .original_file_path("original_file_path".to_string())
            .alias("alias".to_string())
            .model_name("model_name".to_string())
            .build()
            .unwrap();

        assert_eq!(node_metadata.unique_id, "unique_id".to_string());
        assert_eq!(node_metadata.resource_type, DbtResourceType::Model);
        assert_eq!(
            node_metadata.dbt_package_name,
            "dbt_package_name".to_string()
        );
        assert_eq!(node_metadata.model_path, "model_path".to_string());
        assert_eq!(
            node_metadata.original_file_path,
            "original_file_path".to_string()
        );
        assert_eq!(node_metadata.alias, "alias".to_string());
        assert_eq!(node_metadata.model_name, "model_name".to_string());
    }
}
