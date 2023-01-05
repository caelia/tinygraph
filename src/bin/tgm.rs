#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Args, Action};
use tinygraph::sqlite::database::SqliteDatabase;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let tgm = Tgm::new();
    match args.action {
        Action::Init => {
            let full_path = match args.path {
                Some(p) => {
                    let path = PathBuf::from(p);
                    let dbname = match args.name {
                        Some(name) => name,
                        None => tgm.base.default_name(),
                    };
                    let fp = path.join(dbname);
                    match (fp.exists(), args.replace) {
                        (true, true) => {
                            match std::fs::remove_file(&fp) {
                                Ok(_) => fp,
                                Err(e) => panic!("{:?}", e),
                            }
                        },
                        (true, false) => {
                            panic!("Database already exists.");
                        }
                        (false, _) => fp,
                    }
                }
                None => {
                    let path = match tgm.base.default_dir() {
                        Some(p) => PathBuf::from(p),
                        None => {
                            match std::env::current_dir() {
                                Ok(dir) => dir,
                                Err(_) => panic!("Can't obtain current directory."),
                            }
                        }
                    };
                    match args.name {
                        Some(name) => path.join(name),
                        None => path.join(tgm.base.default_name()),
                    }
                }
            };
            let parent_dir = full_path.parent().unwrap();
            if !parent_dir.exists() {
                match std::fs::create_dir_all(&parent_dir) {
                    Ok(_) => (),
                    Err(e) => panic!("{:?}", e),
                }
            }
            let _ = SqliteDatabase::new(None, true, args.replace, vec![]);
        },
        Action::Query => println!("OK, let's do a query!")
    }
}
