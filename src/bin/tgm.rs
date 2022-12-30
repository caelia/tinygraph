#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;
use structopt::StructOpt;
use tinygraph::Database;
use tinygraph::cli::{Tgm, Command};

fn main() {
    let tgm = Tgm::from_args();
    /*
    if let (path, cmd): (Option<String>, Command) = tgm {
        Tgm { path. cmd }
    }
    >*/
    println!("TGM: {:?}", tgm );
}
