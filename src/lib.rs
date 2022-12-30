#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod cli;

use rusqlite::{params, Connection};
use rusqlite::Result as RsqResult;
use rusqlite::Error as RsqError;
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
pub enum DbOptions {
    Read,
    Write,
}

#[derive(Debug)]
pub struct Database<'a> {
    // path: &'a Path,
    path: &'a PathBuf,
    conn: Option<Connection>,
    options: Vec<DbOptions>,
}

const Q_CREATE_REL_TABLE: &str = "
    CREATE TABLE relations (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        relname TEXT NOT NULL,
        bidi INTEGER,
        inverse TEXT
    );
";

const Q_CREATE_TYPE_TABLE: &str = "
    CREATE TABLE types (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        typename TEXT NOT NULL
    );
";

const Q_CREATE_DATA_TABLE: &str = "
    CREATE TABLE data (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        node_id INTEGER,
        relation REFERENCES relations(id) NOT NULL,
        type REFERENCE types(id) NOT NULL,
        value TEXT NOT NULL
    );
";

impl<'a> Database<'a> {
    // pub fn new(path: &'a Path, name: Option<String>, init: bool,
    pub fn new(path: &'a PathBuf, name: Option<String>, init: bool,
            overwrite: bool, create_path: bool, options: Vec<DbOptions>)
            -> Result<Self, Box<dyn std::error::Error>> {
        let filename = match name {
            // Some(fname) => Path::new(&fname),
            Some(fname) => PathBuf::from(&fname),
            // None => Path::new("tgr_data.db"),
            None => PathBuf::from("tgr_data.db"),
        };
        let full_path = path.join(filename);
        let ok = if init {
            // match Self::initialize(&full_path, overwrite) {
            match Self::initialize(full_path.clone(), overwrite) {
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
    
    // fn initialize(path: &'a Path, overwrite: bool) -> Result<(), Box<dyn std::error::Error>> {
    fn initialize(path: PathBuf, overwrite: bool) -> Result<(), Box<dyn std::error::Error>> {
        if &path.exists() & !overwrite {
            panic!("DB file '{:?}' already exists.", &path);
        }
        Ok(())
    }
    
    // fn setup

    pub fn open(&mut self) {
        if let Ok(konn) = Connection::open(self.path) {
            self.conn = Some(konn);
        }
    }
    
    pub fn connect(&mut self) {
    }

    // Should return a Result
    pub fn close(&mut self) -> RsqResult<(), (Connection, RsqError)> {
        let conn = self.conn.take();
        if let Some(konn) = conn {
            konn.close()
        } else {
            Ok(())
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
