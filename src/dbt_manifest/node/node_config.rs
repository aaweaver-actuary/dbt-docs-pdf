use crate::types::DbtMaterialization;

use derive_builder::Builder;
use std::fmt;

#[derive(Debug, Clone, Builder)]
pub struct NodeConfig {
    pub is_enabled: bool,
    pub alias: Option<String>,
    pub schema: String,
    pub database: Option<String>,
    pub tags: Vec<String>,
    pub meta: Option<String>,
    pub group: Option<String>,
    pub materialized: Option<DbtMaterialization>,
}
