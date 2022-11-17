#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod cli;

use rusqlite::{params, Connection, Result};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
// use std::rc::Rc;

#[derive(Debug)]
struct TinyGraphError {
    desc: String,
}

impl fmt::Display for TinyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TinyGraph Error!!!!!!!!!!!!!!!!")
    }
}

impl Error for TinyGraphError {}


#[derive(Debug)]
enum LiteralType {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug)]
enum Target {
    Literal(LiteralType),
    IRef(u32),
    XRef(String),
    RRef(u32),
}

#[derive(Debug)]
struct Edge {
    id: u32,
    label: String,
    target: Target
}

#[derive(Debug)]
enum DbOptions {
    Read,
    Write,
}

#[derive(Debug)]
pub struct Database<'a> {
    path: &'a Path,
    conn: Option<Connection>,
    options: Vec<DbOptions>,
}

impl<'a> Database<'a> {
    pub fn new(path: &'a Path, name: Option<String>, init: bool,
            overwrite: bool, create_path: bool, options: Vec<DbOptions>)
            -> Result<Self, Box<dyn std::error::Error>> {
        let filename = match name {
            Some(fname) => Path::new(&fname),
            None => Path::new("tgr_data.db"),
        };
        let full_path = path.join(filename);
        let ok = if init {
            match Self::initialize(full_path, overwrite).unwrap() {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        };
        if ok {
            Ok(Database { path: &path, conn: None, options })
        } else {
            Err(Box::new(TinyGraphError { desc: String::from("Unable to create database!")}))
        }
    }
    
    fn initialize(path: &'a Path, overwrite: bool) -> Result<(), Box<dyn std::error::Error>> {
        if &path.exists() & !overwrite {
            panic!("DB file '{:?}' already exists.", &path);
        }
        Ok(())
    }

    pub fn open(&mut self) {
        if let Ok(konn) = Connection::open(self.path) {
            self.conn = Some(konn);
        }
    }
    
    pub fn connect(&mut self) {
    }

    // Should return a Result
    pub fn close(&mut self) {
        let conn = self.conn.take();
        if let Some(konn) = conn {
            konn.close();
        }
    }
}

    
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
