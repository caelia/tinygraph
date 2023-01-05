#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::tg_error;
use tinygraph::error::TinyGraphError;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Args, Action};
use tinygraph::app::App;
use tinygraph::sqlite::database::SqliteDatabase;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let tgm = Tgm::new();
    match args.action {
        Action::Init => {
            let full_path = match args.path {
                Some(p) => {
                    let path = PathBuf::from(p);
                    let dbname = match args.name {
                        Some(name) => name,
                        None => {
                            match tgm.default_name() {
                                Ok(name) => name,
                                Err(e) => return tg_error!("{:?}", e),
                            }
                        }
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
                    let path = match tgm.default_dir() {
                        Ok(Some(p)) => p,
                        Ok(None) => {
                            match std::env::current_dir() {
                                Ok(dir) => dir,
                                Err(_) => return tg_error!("Can't obtain current directory."),
                            }
                        },
                        Err(e) => return tg_error!("{:?}", e)
                    };
                    match args.name {
                        Some(name) => path.join(name),
                        None => {
                            match tgm.default_name() {
                                Ok(name) => path.join(name),
                                Err(e) => return tg_error!("{:?}", e)
                            }
                        }
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
            let opts = match parent_dir.to_str() {
                Some(str) => vec![("path".to_string(), str.to_string())],
                None => return tg_error!("Can't convert path to string: '{:?}", parent_dir),
            };
            match SqliteDatabase::new(None, true, args.replace, opts) {
                Ok(_) => {
                    println!("Database created.");
                    Ok(())
                },
                Err(e) => tg_error!("{:?}", e)
            }
        },
        Action::Query => {
            println!("OK, let's do a query!");
            Ok(())
        }
    }
}
