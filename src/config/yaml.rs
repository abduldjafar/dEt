// Cargo.toml
// [dependencies]
// serde = { version = "1", features = ["derive"] }
// serde_yaml = "0.9"
// indexmap = { version = "2", features = ["serde"] }
// thiserror = "1"

use indexmap::IndexMap;
use serde::Deserialize;
use std::borrow::Cow;

/* ========================== Top-level ========================== */

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DetConfig<'a> {
    pub name: Cow<'a, str>,
    pub profile: Cow<'a, str>,
    pub extract: Extract<'a>,
    pub transform: Transform<'a>,
    pub load: Load<'a>,
}

/* ========================== Sections =========================== */

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct Extract<'a> {
    // borrow keys and values from the YAML buffer
    #[serde(borrow)]
    pub sources: IndexMap<Cow<'a, str>, SourceConnector<'a>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct Transform<'a> {
    pub engine: Engine, // must be datafusion
    #[serde(borrow)]
    pub sql_paths: Vec<Cow<'a, str>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct Load<'a> {
    #[serde(borrow)]
    pub destinations: Vec<DestinationConnector<'a>>,
}

/* ============================ Enums ============================ */

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Datafusion,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub enum SourceConnector<'a> {
    Filesystem(FilesystemSource<'a>),
    // Postgres(PostgresSource<'a>),
    // S3(S3Source<'a>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub enum DestinationConnector<'a> {
    Filesystem(FilesystemDestination<'a>),
    Postgres(PostgresDestination<'a>),
    // Clickhouse(ClickhouseDestination<'a>),
    // S3(S3Destination<'a>),
}

/* ======================== Source payloads ====================== */

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct FilesystemSource<'a> {
    pub format: FileFormat,
    pub path: Cow<'a, str>,
}

/* ===================== Destination payloads ==================== */

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct FilesystemDestination<'a> {
    pub name: Cow<'a, str>,
    pub base_dir: Cow<'a, str>,
    pub format: FileFormat,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct PostgresDestination<'a> {
    pub name: Cow<'a, str>,
    pub dsn: Cow<'a, str>,
    #[serde(default)]
    pub write_mode: Option<WriteMode>,
    #[serde(default)]
    pub schema: Option<Cow<'a, str>>,
}

/* =========================== Shared ============================ */

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Parquet,
    Csv,
    Json,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    InsertAppend,
    InsertOverwrite,
    Merge,
}

/* ==================== Parse & Validate ========================= */

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("yaml parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("invalid engine: only `datafusion` is supported")]
    EngineUnsupported,
    #[error("no destinations configured")]
    NoDestinations,
    #[error("no sources configured")]
    NoSources,
}

pub fn parse_det_config<'a>(yaml: &'a str) -> Result<DetConfig<'a>, ConfigError> {
    let cfg: DetConfig<'a> = serde_yaml::from_str(yaml)?;
    validate(&cfg)?;
    Ok(cfg)
}

fn validate(cfg: &DetConfig<'_>) -> Result<(), ConfigError> {
    if cfg.transform.engine != Engine::Datafusion {
        return Err(ConfigError::EngineUnsupported);
    }
    if cfg.extract.sources.is_empty() {
        return Err(ConfigError::NoSources);
    }
    if cfg.load.destinations.is_empty() {
        return Err(ConfigError::NoDestinations);
    }
    Ok(())
}
