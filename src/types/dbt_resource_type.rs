use std::fmt::{self, Display, Formatter};

/// Represents the type of a dbt resource.
///
/// This enum is used to represent the type of a dbt resource, such as a model, macro, test, seed, source, or snapshot.
///
/// # Example
///
/// ```
/// use dbt_docs_pdf::types::dbt_resource_type::DbtResourceType;
///
/// let resource = DbtResourceType::Model;
/// assert_eq!(resource.to_string(), "model");
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DbtResourceType {
    #[default]
    Model,
    Macro,
    Test,
    Seed,
    Source,
    Snapshot,
}

impl Display for DbtResourceType {
    /// Formats the dbt resource type as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use dbt_docs_pdf::types::dbt_resource_type::DbtResourceType;
    ///
    /// let resource = DbtResourceType::Model;
    /// assert_eq!(resource.to_string(), "model");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbtResourceType::Model => write!(f, "model"),
            DbtResourceType::Macro => write!(f, "macro"),
            DbtResourceType::Test => write!(f, "test"),
            DbtResourceType::Seed => write!(f, "seed"),
            DbtResourceType::Source => write!(f, "source"),
            DbtResourceType::Snapshot => write!(f, "snapshot"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_model() {
        let resource = DbtResourceType::Model;
        assert_eq!(resource.to_string(), "model");
    }

    #[test]
    fn test_display_macro() {
        let resource = DbtResourceType::Macro;
        assert_eq!(resource.to_string(), "macro");
    }

    #[test]
    fn test_display_test() {
        let resource = DbtResourceType::Test;
        assert_eq!(resource.to_string(), "test");
    }

    #[test]
    fn test_display_seed() {
        let resource = DbtResourceType::Seed;
        assert_eq!(resource.to_string(), "seed");
    }

    #[test]
    fn test_display_source() {
        let resource = DbtResourceType::Source;
        assert_eq!(resource.to_string(), "source");
    }

    #[test]
    fn test_display_snapshot() {
        let resource = DbtResourceType::Snapshot;
        assert_eq!(resource.to_string(), "snapshot");
    }

    #[test]
    fn test_display_default() {
        let resource = DbtResourceType::default();
        assert_eq!(resource.to_string(), "model");
    }
}
