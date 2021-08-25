use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::remove_file;
use tinygraph::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "tgm", about = "TinyGraph Manager CLI app")]
enum Options {
    /// Initialize a TinyGraph data store.
    Init {
        /// If a file with the given name already exists, replace it.
        #[structopt(short, long)]
        replace: bool,

        /// An absolute or relative file path for the DB file. If no value is provided,
        /// the file created will be named `tinygraph.db` in the current directory.
        #[structopt(parse(from_os_str))]
        filename: Option<PathBuf>,
    },
    Query {
    },
    Add {
    },
    Delete {
    }
}

fn init(path: PathBuf, replace: bool) {
    // println!("{:?}, {}", path, replace);
    if path.exists() {
        if replace {
            remove_file(&path);
        } else {
            panic!("DB file '{:?}' already exists.", path);
        }
    }
    let mut store: SqliteTGStore = SqliteTGStore::from_path(path);
    // let store = SqliteTGStore::new();
    store.connect();
    store.setup();
    store.disconnect();
    // println!("TG Store: {:?}", store);
}

fn main() {
    let options = Options::from_args();
    match options {
        Options::Init {replace, filename} => match filename {
            Some(path) => init(path, replace),
            None => init(PathBuf::from("./tinygraph.db"), replace),
        },
        Options::Query {} => println!("Query!"),
        Options::Add {} => println!("Add!"),
        Options::Delete {} => println!("Delete!"),
    }
}
