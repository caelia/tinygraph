use crate::tg_error;
use crate::tgconfig::Config;
use crate::app::App;
use crate::error::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

pub struct Tgm {
    config: Config,
}

impl App for Tgm {
    fn new(config: Config) -> Self {
        Tgm { config }
    }
    fn default_dir(&self) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        match self.config_get("default_directory".to_string()) {
            Some(dir) => Ok(Some(PathBuf::from(dir))),
            None => match std::env::current_dir() {
                Ok(dir) => Ok(Some(PathBuf::from(dir))),
                Err(e) => tg_error!("{:?}", e)
            }
        }
    }
    fn default_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.config_get("default_name".to_string()) {
            Some(name) => Ok(name),
            None => Ok("tg_data.db".to_string()),
        }
    }
    fn config_get(&self, key: String) -> Option<String> {
        self.config.get(key)
    }
    fn config_set(&mut self, key: String, value: String) {
        self.config.set(key, value);
    }
}
