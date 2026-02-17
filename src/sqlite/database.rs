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
use std::fs::create_dir_all;
use crate::sqlite::sql;
use crate::error::*;
use crate::tgconfig;


struct SqliteDbOptions {
    data: HashMap<String, String>,
}

impl SqliteDbOptions {
    fn new(options: Vec<(String, String)>) -> Self {
        let mut data = HashMap::new();
        for (k, v) in options {
            assert!(SqliteDbOptions::valid_item(&k, &v));
            let _ = data.insert(k, v);
        }
        SqliteDbOptions { data }
    }

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

    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        if Self::valid_item(&key, &value) {
            let _ = self.data.insert(key, value);
            Ok(())
        } else {
            tg_error!("Invalid database option: {}: {:?}", key, value)
        }
    }

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
    path: PathBuf,
    conn: Option<Connection>,
    options: HashMap<String,String>,
}


impl SqliteDatabase {
    pub fn new(dir: PathBuf, fname: String, init: bool, replace: bool,
           options: Vec<(String, String)>)
            -> Result<Self, Box<dyn std::error::Error>> {
        // let config = tgconfig::Config::new();
        let opts = SqliteDbOptions::new(options);
        let full_path = dir.join(&fname);
        let db = if init {
            let mut db_ = match Self::initialize(dir.clone(), fname.clone(), replace) {
                Ok(konn) => {
                    SqliteDatabase {
                        conn: Some(konn),
                        path: full_path,
                        options: opts.data.clone()
                    }
                },
                Err(e) => {
                    return tg_error!("No database connection.\n{:?}", e);
                }
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

    fn initialize(dir: PathBuf, fname: String,  replace: bool) -> Result<Connection, Box<dyn std::error::Error>> {
        let full_path = dir.join(fname);
        if full_path.exists() {
            if replace {
                match std::fs::remove_file(&full_path) {
                    Ok(_) => (),
                    Err(_) => {
                        return tg_error!("Unable to remove existing database.");
                    }
                }
            } else {
                return tg_error!("Database '{:?}' already exists.", &full_path);
            }
        } else if !dir.exists() {
            match create_dir_all(dir) {
                Ok(_) => (),
                Err(e) => return tg_error!("Can't create database directory. {:?}", e),
            }
        }
        match Connection::open_with_flags(full_path,
                RsqOpenFlags::SQLITE_OPEN_READ_WRITE
                |RsqOpenFlags::SQLITE_OPEN_CREATE) {
            Ok(conn) => Ok(conn),
            Err(e) => {
                tg_error!("Unable to create SQLite database.\n{:?}", e)
            }
        }
    }
    
    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let err_msg : String;
        match &self.conn {
            Some(konn) => {
                if let Err(e) = konn.execute(sql::CREATE_REL_TABLE, ()) {
                    return tg_error!("Database setup failed on relation table creation query.\n{:?}", e);
                }
                if let Err(e) = konn.execute(sql::CREATE_TYPE_TABLE, ()) {
                    return tg_error!("Database setup failed on type table creation query.\n{:?}", e);
                }
                if let Err(e) = konn.execute(sql::CREATE_DATA_TABLE, ()) {
                    return tg_error!("Database setup failed on data table creation query.\n{:?}", e);
                }
                // Typenames include 4 types of references:
                // nref [node reference] - references a node in the same db
                // rref [reifying reference] - references an edge in the same db
                // xref [cross-reference] - references an item in another local db
                // uref [uri reference] - flexible, but mainly for referring to remote dbs
                for typename in ["int", "float", "bool", "datetime", "string",
                                 "nref", "rref", "xref", "uref"] {
                    if let Err(e) = konn.execute(sql::POPULATE_TYPE_TABLE, [typename]) {
                        return tg_error!("Database setup failed attempting to populate type table.\n{:?}", e);
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
