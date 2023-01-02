#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Command};
use clap::Parser;

fn main() {
    let tgm = Tgm::parse();
    match tgm.cmd {
        Command::Init => {
            match tgm.path {
                Some(p) => println!("Create Tinygraph DB at {}.", p),
                None => println!("Create Tinygraph DB in current directory.")
            }
        },
        _ => panic!("Unknown command - this should never happen.")
    }
}
