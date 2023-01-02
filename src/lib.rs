#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod cli;
pub mod query;

use rusqlite::{params, Connection};
use rusqlite::Result as RsqResult;
use rusqlite::Error as RsqError;
use rusqlite::OpenFlags as RsqOpenFlags;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;

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

impl TinyGraphError {
    fn new(desc: String) -> Self {
        TinyGraphError { desc }
    }
    fn default() -> Self {
        TinyGraphError { desc: String::from("Ruh-roh!") }
    }
}

macro_rules! tg_error {
    ($($args:tt)*) => {
        Err(Box::new(TinyGraphError::new(format!($($args)*))))   
    }
}


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
    path: &'a PathBuf,
    conn: Option<Connection>,
    options: Vec<DbOptions>,
}


impl<'a> Database<'a> {
    pub fn new(path: &'a PathBuf, name: Option<String>, init: bool,
            overwrite: bool, create_path: bool, options: Vec<DbOptions>)
            -> Result<Self, Box<dyn std::error::Error>> {
        let filename = match name {
            Some(fname) => PathBuf::from(&fname),
            None => PathBuf::from("tgr_data.db"),
        };
        let full_path = path.join(filename);
        let conn = if init {
            match Self::initialize(full_path.clone(), overwrite) {
                Ok(konn) => Some(konn),
                Err(_) => return tg_error!("No database connection.")
            }
        } else {
            None
        };
        match conn {
            Some(_) => Ok(Database { path: &path, conn, options }),
            None => tg_error!("Unable to create database object!")
        }
    }
    
    fn initialize(path: PathBuf, overwrite: bool) -> Result<Connection, Box<dyn std::error::Error>> {
        if path.exists() {
            if overwrite {
                match std::fs::remove_file(&path) {
                    Ok(_) => (),
                    Err(_) => {
                        return tg_error!("Unable to remove existing database.");
                    }
                }
            } else {
                return tg_error!("Database '{:?}' already exists.", &path);
            }
        }
        match Connection::open_with_flags(path, RsqOpenFlags::SQLITE_OPEN_CREATE) {
            Ok(conn) => Ok(conn),
            Err(_) => {
                tg_error!("Unable to create SQLite database.")
            }
        }
    }
    
    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let err_msg : String;
        match &self.conn {
            Some(konn) => {
                if let Err(_) = konn.execute(query::CREATE_REL_TABLE, ()) {
                    return tg_error!("Database setup failed on relation table creation query.");
                }
                if let Err(_) = konn.execute(query::CREATE_TYPE_TABLE, ()) {
                    return tg_error!("Database setup failed on type table creation query.");
                }
                if let Err(_) = konn.execute(query::CREATE_DATA_TABLE, ()) {
                    return tg_error!("Database setup failed on data table creation query.");
                }
                for typename in ["int", "float", "bool", "datetime",
                                 "string", "iref", "lref", "rref"] {
                    if let Err(_) = konn.execute(query::POPULATE_TYPE_TABLE, [typename]) {
                        return tg_error!("Database setup failed attempting to populate type table.");
                    }
                }
                Ok(())
            },
            None => {
                tg_error!("Database setup failed - no connection.")
            }
        }
    }

    pub fn open(&mut self) {
        if let Ok(konn) = Connection::open(self.path) {
            self.conn = Some(konn);
        }
    }
    
    pub fn connect(&mut self) {
    }

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
