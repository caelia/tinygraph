use crate::tg_error;
use crate::tgconfig::Config;
use crate::app::App;
use crate::error::*;
use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct MainCommand {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Initialize a database
    Init(InitArgs),
    /// Perform a query
    Query(QueryArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Database filename
    #[arg(long, short)]
    pub name: Option<String>,
    /// Directory where DB file is stored
    #[arg(long, short)]
    pub path: Option<String>,
    /// If DB already exists, replace it with new file
    #[arg(long, short)]
    pub replace: bool,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    
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
