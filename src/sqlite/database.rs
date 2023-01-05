#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rusqlite::{params, Connection};
use rusqlite::Result as RsqResult;
use rusqlite::Error as RsqError;
use rusqlite::OpenFlags as RsqOpenFlags;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use crate::sqlite::sql;
use crate::error::*;
use crate::tgconfig;
// use crate::{Database, DbOptions};
use crate::Database;

/*
#[derive(Debug)]
enum SqliteOption {
    Bool(bool),
    String(std::string::String),
    PathBuf(std::path::PathBuf),
}
*/

struct SqliteDbOptions {
    data: HashMap<String, String>,
}

// impl DbOptions<String> for SqliteDbOptions {
impl SqliteDbOptions {
    fn new(options: Vec<(String, String)>) -> Self {
        let mut data = HashMap::new();
        for (k, v) in options {
            assert!(SqliteDbOptions::valid_item(&k, &v));
            let _ = data.insert(k, v);
        }
        SqliteDbOptions { data }
    }
    // fn get(&self, key: String) -> Result<Option<SqliteOption>, Box<dyn std::error::Error>> {
    fn get(&self, key: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if Self::valid_key(&key) {
            match self.data.get(&key) {
                Some(opt) => Ok(Some(opt.clone())),
                None => Ok(None)
            }
        } else {
            tg_error!("invalid database option: '{}'", key)
        }
    }
    // fn set(&self, key: String, value: SqliteOption) -> Result<(), Box<dyn std::error::Error>> {
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        if Self::valid_item(&key, &value) {
            let _ = self.data.insert(key, value);
            Ok(())
        } else {
            tg_error!("Invalid database option: {}: {:?}", key, value)
        }
    }
/*
}

impl SqliteDbOptions {
    */
    fn valid_key(key: &str) -> bool {
        match key {
            "read"|"write"|"path"|"create_dir" => true,
            _ => false,
        }
    }
    fn valid_item(key: &str, value: &str) -> bool {
        match (key, value) {
            ("read", "true")
            | ("write", "true")
            | ("create_dir", "true")
            | ("read", "false")
            | ("write", "false")
            | ("create_dir", "false")  => true,
            ("path", _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct SqliteDatabase {
    // path: &'a PathBuf,
    path: PathBuf,
    conn: Option<Connection>,
    options: HashMap<String,String>,
}


// impl<'a> Database<'a> {
// impl<DBO: DbOptions> Database<DBO: DbOptions> for SqliteDatabase {
impl Database for SqliteDatabase {
    fn new(name: Option<String>, init: bool, replace: bool,
           options: Vec<(String, String)>)
            -> Result<Self, Box<dyn std::error::Error>> {
        let config = tgconfig::Config::new();
        let subdir = match name {
            Some(ref dname) => dname.clone(),
            None => "tinygraph_db".to_string(),
        };
        let opts = SqliteDbOptions::new(options);
        let dir = match opts.get("path".to_string()) {
            Ok(Some(path_)) => PathBuf::from(path_),
            Ok(None) => {
                match config.get("sqlite_default_directory".to_string()) {
                    Some(dir_) => PathBuf::from(dir_),
                    None => return tg_error!("No directory for Sqlite database."),
                }
            },
            Err(e) => return tg_error!("{:?}", e),
        };
        let filename = match name {
            Some(fname) => PathBuf::from(&fname),
            None => PathBuf::from("tgr_data.db"),
        };
        let full_path = dir.join(filename);
        let db = if init {
            let mut db_ = match Self::initialize(full_path.clone(), replace) {
                Ok(konn) => {
                    SqliteDatabase {
                        conn: Some(konn),
                        path: full_path,
                        options: opts.data.clone()
                    }
                },
                Err(_) => return tg_error!("No database connection.")
            };
            match db_.setup() {
                Ok(_) => db_,
                Err(e) => return tg_error!("{:?}", e)
            }
        } else {
            match Connection::open(&full_path) {
                Ok(konn) => {
                    SqliteDatabase {
                        conn: Some(konn),
                        path: full_path,
                        options: opts.data.clone() }
                },
                Err(_) => return tg_error!("Unable to connect to SQLite db.")
            }
        };
        Ok(db)
    }
    
    fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match Connection::open(&self.path) {
            Ok(konn) => {
                self.conn = Some(konn);
                Ok(())
            },
            Err(e) => tg_error!("{:?}", e)
        }
    }
    
    /*

    struct SqliteDbOptions {
        
    }
    fn connect(&mut self) {
    }
    */

    // fn close(&mut self) -> RsqResult<(), (Connection, RsqError)> {
    fn close(&mut self) -> RsqResult<(), Box<dyn std::error::Error>> {
        let conn = self.conn.take();
        if let Some(konn) = conn {
            match konn.close() {
                Ok(_) => Ok(()),
                Err((_, e)) => Err(Box::new(e))
            }
        } else {
            tg_error!("Attempted to close a nonexistent connection. This should never happen.")
        }
    }
}

impl SqliteDatabase {
  fn initialize(path: PathBuf, replace: bool) -> Result<Connection, Box<dyn std::error::Error>> {
        if path.exists() {
            if replace {
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
                if let Err(_) = konn.execute(sql::CREATE_REL_TABLE, ()) {
                    return tg_error!("Database setup failed on relation table creation query.");
                }
                if let Err(_) = konn.execute(sql::CREATE_TYPE_TABLE, ()) {
                    return tg_error!("Database setup failed on type table creation query.");
                }
                if let Err(_) = konn.execute(sql::CREATE_DATA_TABLE, ()) {
                    return tg_error!("Database setup failed on data table creation query.");
                }
                for typename in ["int", "float", "bool", "datetime",
                                 "string", "iref", "lref", "rref"] {
                    if let Err(_) = konn.execute(sql::POPULATE_TYPE_TABLE, [typename]) {
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
}
