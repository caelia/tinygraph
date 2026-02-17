#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::tg_error;
use tinygraph::error::TinyGraphError;
use tinygraph::cli::{Tgm, Args, Action};
use tinygraph::app::App;
use tinygraph::sqlite::database::SqliteDatabase;
use tinygraph::tgconfig::Config;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let tgm = Tgm::new(Config::default());
    match args.action {
        Action::Init => {
            let (dir, fname) = match args.path {
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
                    (path, dbname)
                },
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
                        Some(name) => (path, name),
                        None => {
                            match tgm.default_name() {
                                Ok(name) => (path, name),
                                Err(e) => return tg_error!("{:?}", e)
                            }
                        }
                    }
                }
            };
            let opts = vec![];
            match SqliteDatabase::new(dir, fname, true, args.replace, opts) {
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
