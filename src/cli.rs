use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Tgm {
    #[arg(long, short)]
    pub path: Option<String>,
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
}
