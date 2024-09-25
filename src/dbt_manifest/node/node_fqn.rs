use std::fmt;

/// NodeFqn struct represents the fully qualified name of a node
/// in the dbt manifest: db_name.schema_name.model_name
///
/// # Example
/// ```
/// use dbt_docs_pdf::dbt_manifest::node::node_fqn::NodeFqn;
///
/// let db_name = "db_name".to_string();
/// let schema_name = "schema_name".to_string();
/// let model_name = "model_name".to_string();
///
/// let node_fqn = NodeFqn {
///     db_name: Some(db_name),
///     schema_name: Some(schema_name),
///     model_name: Some(model_name),
/// };
///
/// assert_eq!(node_fqn.db_name, Some("db_name".to_string()));
/// assert_eq!(node_fqn.schema_name, Some("schema_name".to_string()));
/// assert_eq!(node_fqn.model_name, Some("model_name".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct NodeFqn {
    pub db_name: Option<String>,
    pub schema_name: Option<String>,
    pub model_name: Option<String>,
}

impl fmt::Display for NodeFqn {
    /// Display implementation for NodeFqn
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.db_name.as_ref().unwrap(),
            self.schema_name.as_ref().unwrap(),
            self.model_name.as_ref().unwrap()
        )
    }
}

impl NodeFqn {
    pub fn new(
        db_name: Option<String>,
        schema_name: Option<String>,
        model_name: Option<String>,
    ) -> Self {
        NodeFqn {
            db_name,
            schema_name,
            model_name,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    /// Test that the NodeFqnBuilder can be used to build a NodeFqn struct
    /// Expected: NodeFqn struct is built with the correct values of db_name,
    /// schema_name, and model_name
    #[test]
    fn test_node_fqn_builder() {
        let node_fqn = NodeFqn::new(
            Some("db_name".to_string()),
            Some("schema_name".to_string()),
            Some("model_name".to_string()),
        );

        assert_eq!(node_fqn.db_name, Some("db_name".to_string()));
        assert_eq!(node_fqn.schema_name, Some("schema_name".to_string()));
        assert_eq!(node_fqn.model_name, Some("model_name".to_string()));
    }
}
