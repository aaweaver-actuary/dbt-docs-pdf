use std::fmt;

#[derive(Debug, Clone)]
pub enum DbtMaterialization {
    View,
    Table,
    Incremental,
    Ephemeral,
}

impl fmt::Display for DbtMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbtMaterialization::View => write!(f, "view"),
            DbtMaterialization::Table => write!(f, "table"),
            DbtMaterialization::Incremental => write!(f, "incremental"),
            DbtMaterialization::Ephemeral => write!(f, "ephemeral"),
        }
    }
}
