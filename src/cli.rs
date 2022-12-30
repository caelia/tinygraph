use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
// #[structopt(name = "basic")]
pub struct Tgm {
    #[arg(long, short)]
    path: Option<String>,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
}
