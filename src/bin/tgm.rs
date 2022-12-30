#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Command};
use clap::Parser;

fn main() {
    let tgm = Tgm::parse();
    /*
    if let (path, cmd): (Option<String>, Command) = tgm {
        Tgm { path. cmd }
    }
    >*/
    println!("TGM: {:?}", tgm );
}
