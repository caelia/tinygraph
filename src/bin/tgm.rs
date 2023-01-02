#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Args, Action};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let tgm = Tgm::new();
    match args.action {
        Action::Init => {
            match args.path {
                Some(p) => {
                    // Check whether path exists ...
                    // If yes,
                    // If no,
                }
                None => println!("Create Tinygraph DB in current directory.")
            }
        },
        Action::Query => println!("OK, let's do a query!")
    }
}
