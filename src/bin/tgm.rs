use tinygraph::*;
use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use std::fs::remove_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a TinyGraph data store.
    Init(InitArgs),
    Query(QueryArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Dump(DumpArgs),
}

#[derive(Args, Debug)]
struct InitArgs {
    /// If a file with the given name already exists, replace it.
    #[arg(short, long)]
    replace: bool,

    /// An absolute or relative file path for the DB file. If no value is provided,
    /// the file created will be named `tinygraph.db` in the current directory.
    #[arg(short, long)]
    filename: Option<PathBuf>,
}
#[derive(Args, Debug)]
struct QueryArgs {
    
}
#[derive(Args, Debug)]
struct AddArgs {
    
}
#[derive(Args, Debug)]
struct RemoveArgs {
    
}
#[derive(Args, Debug)]
struct DumpArgs {
    
}

fn init(path: PathBuf, replace: bool) {
    // println!("{:?}, {}", path, replace);
    if path.exists() {
        if replace {
            let _ = remove_file(&path);
        } else {
            panic!("DB file '{:?}' already exists.", path);
        }
    }
    let mut store: SqliteTGStore = SqliteTGStore::from_path(path);
    // let store = SqliteTGStore::new();
    let _ = store.connect();
    let _ = store.setup();
    let _ = store.disconnect();
    // println!("TG Store: {:?}", store);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(init_args) => {
            let (path, replace) = match init_args {
                InitArgs { filename: Some(name), replace: r } => {
                    (name, r)
                },
                InitArgs { filename: None, replace: r } => {
                    (PathBuf::from("./tinygraph.db"), r)
                },
            };
            init(path, replace);
        },
        Commands::Query(_) => println!("Query!"),
        Commands::Add(_) => println!("Add!"),
        Commands::Remove(_) => println!("Remove!"),
        Commands::Dump(_) => println!("Dump!"),
    }
}
