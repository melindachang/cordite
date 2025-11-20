use crate::cli::{
    add::{AddFileAction, AddStrategy},
    sync::SyncStrategy,
};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct AddConfig {
    pub auto_tag: bool,
    pub auto_rename: bool,
    pub file_action: AddFileAction,
    pub strategy: AddStrategy,
    pub write_metadata: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct SyncConfig {
    pub strategy: SyncStrategy,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Commands {
    pub add: AddConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Renamer {
    template: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub debug: bool,
    pub library_path: PathBuf,
    pub metadata_db_path: PathBuf,
    pub commands: Commands,
    pub renamer: Renamer,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(File::with_name("examples/config/default"))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        cfg.try_deserialize()
    }
}
