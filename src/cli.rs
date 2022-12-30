use structopt::StructOpt;

#[derive(StructOpt, Debug)]
// #[structopt(name = "basic")]
pub struct Tgm {
    #[structopt(long, short)]
    path: Option<String>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Init,
}
