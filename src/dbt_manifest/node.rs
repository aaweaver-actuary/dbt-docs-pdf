use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Encapsulating a fully qualified name as its own type
#[derive(Serialize, Deserialize, Debug)]
pub struct Fqn(pub Vec<String>);

/// Enum for resource type field
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Model,
    Seed,
    Snapshot,
    Test,
    Analysis,
    Other(String),
}

/// Enum for access control
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    Protected,
    Public,
    Private,
}

/// Enum for materialization strategies
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Materialization {
    View,
    Table,
    Incremental,
    Ephemeral,
    Other(String),
}

/// Encapsulating SQL code as a distinct type
#[derive(Serialize, Deserialize, Debug)]
pub struct SqlCode(pub String);

/// Encapsulating Meta information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Meta {
    pub owner: Option<String>,
    pub team: Option<String>,
}

/// Encapsulating Grants
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grants {
    pub read: Vec<String>,
    pub write: Vec<String>,
}

/// Encapsulating Config structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub enabled: bool,
    pub alias: Option<String>,
    pub schema: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: Meta,
    pub materialized: Materialization,
    #[serde(rename = "post-hook", default)]
    pub post_hook: Vec<String>,
    #[serde(rename = "pre-hook", default)]
    pub pre_hook: Vec<String>,
    #[serde(default)]
    pub grants: Grants,
}

/// Encapsulating Node structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub database: Option<String>,
    pub schema: String,
    pub name: String,
    pub resource_type: ResourceType,
    pub package_name: String,
    pub path: String,
    pub unique_id: String,
    pub fqn: Fqn,
    pub alias: Option<String>,
    pub checksum: Checksum,
    pub config: Config,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub columns: HashMap<String, Column>,
    #[serde(default)]
    pub meta: Meta,
    pub relation_name: Option<String>,
    pub raw_code: Option<SqlCode>,
    #[serde(default)]
    pub refs: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
}

/// Encapsulating Checksum structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Checksum {
    pub name: String,
    pub checksum: String,
}

/// Encapsulating Column structure
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Column {
    pub name: Option<String>,
    pub description: Option<String>,
    pub data_type: Option<String>,
}