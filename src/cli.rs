use crate::tg_error;
use crate::tgconfig::Config;
use crate::app::App;
use crate::error::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, short)]
    pub name: Option<String>,
    #[arg(long="type", short='t')]
    pub db_type: Option<String>,
    #[arg(long, short)]
    pub path: Option<String>,
    #[arg(long, short)]
    pub replace: bool,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Init,
    Query,
}

pub struct TgmConfig {
    pub data: HashMap<String, String>,
}

impl Config for TgmConfig {
    type KeyType = String;
    type ValueType = String;

    fn new() -> Self {
        TgmConfig { data: HashMap::new() }
    }
    fn get(&self, key: String) -> Result<Option<&String>, Box<dyn std::error::Error>> {
        let result = self.data.get(&key);
        Ok(result)
    }
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.data.insert(key, value);
        Ok(())
    }
}

impl Default for TgmConfig {
    fn default() -> Self {
        TgmConfig { data: HashMap::new() }
    }
}

pub struct Tgm {
    config: TgmConfig,
}

impl App for Tgm {
    type ConfigType = TgmConfig;
    fn new(config: Self::ConfigType) -> Self {
        Tgm { config }
    }
    fn default_dir(&self) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        match self.config.get("default_directory".to_string()) {
            Ok(Some(dir)) => Ok(Some(PathBuf::from(dir))),
            Ok(None) => match std::env::current_dir() {
                Ok(dir) => Ok(Some(PathBuf::from(dir))),
                Err(e) => tg_error!("{:?}", e)
            },
            Err(e) => tg_error!("{:?}", e)
        }
    }
    fn default_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from("tgr_data.db"))
    }
    /*
    fn config_get(&self, key: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.config.get(key)
    }
    fn config_set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        self.config.set(key, value);
    }
    */
}
