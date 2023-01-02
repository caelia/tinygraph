use crate::tg_error;
use crate::app::App;
use crate::error::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, short)]
    pub path: Option<String>,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Init,
    Query,
}

pub struct Tgm {
    base: App
}

impl Tgm {
    pub fn new() -> Self {
        Tgm { base: App::new() }
    }
    fn default_dir(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        match self.base.default_dir() {
            Some(dir) => Ok(dir),
            None => match std::env::current_dir() {
                Ok(dir) => Ok(dir),
                Err(e) => tg_error!("{:?}", e)
            }
        }
    }
}
