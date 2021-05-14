#![allow(dead_code)]
#![allow(unused_imports)]

use rusqlite::{params, Connection, Result, Error};
use std::path::{Path, PathBuf};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug,Clone)]
struct UnknownError;

impl StdError for UnknownError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown error!")
    }
}

mod sql_queries;
use crate::sql_queries::sqlite3_pgsql::*;

pub trait TGStore {
    type GeneralResult;
    type ConnectionResult;
    // type GeneralError;
    // type ConnectionError;
    fn new() -> Self;
    fn setup(&mut self) -> Self::GeneralResult;
    fn connect(&mut self) -> Self::ConnectionResult;
    fn disconnect(&mut self) -> Self::ConnectionResult;
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
pub struct SqliteTGStore {
    pub path: PathBuf,
    pub conn: Option<Box<Connection>>,
}

impl TGStore for SqliteTGStore {
    type GeneralResult = Result<(), Box<dyn StdError>>;
    type ConnectionResult = Result<(), (Connection, Error)>;
    // type GeneralError = Box<dyn StdError>;
    // type ConnectionError = (Error, Connection);
    fn new() -> Self {
        SqliteTGStore {
            path: PathBuf::new(),
            conn: None
        }
    }
    /*
    pub fn new(path: &PathBuf, overwrite: bool) -> Result<Self, Box<dyn StdError>> {
        if &path.exists() & !overwrite {
            panic!("DB file '{:?}' already exists.", &path);
        }
        let konn = Connection::open(path)?;
        Ok(SqliteTGStore { path: *path, conn: Some(konn) })
    }
    */
    fn setup(&mut self) -> Self::GeneralResult {
        match self.conn.take() {
            None => Err(Box::new(UnknownError)),
            Some(konn) => {
                konn.execute(CREATE_PRIMITIVE_TABLE, NULL_PARAMS).unwrap();
                for q in &POPULATE_PRIMITIVE_TABLE_QQ {
                    konn.execute(q, NULL_PARAMS).unwrap();
                }
                konn.execute(CREATE_STRING_TYPE_TABLE, NULL_PARAMS).unwrap();
                konn.execute(CREATE_NUMBER_TYPE_TABLE, NULL_PARAMS).unwrap();
                konn.execute(CREATE_VOCAB_TABLE, NULL_PARAMS).unwrap();
                konn.execute(CREATE_CARDINALITY_TABLE, NULL_PARAMS).unwrap();
                for q in &POPULATE_CARDINALITY_TABLE_QQ {
                    konn.execute(q, NULL_PARAMS).unwrap();
                }
                konn.execute(CREATE_STRUCT_TYPE_TABLE, NULL_PARAMS).unwrap();
                konn.execute(CREATE_TYPE_CLASS_TABLE, NULL_PARAMS).unwrap();
                for q in &POPULATE_TYPE_CLASS_TABLE_QQ {
                    konn.execute(q, NULL_PARAMS).unwrap();
                }
                konn.execute(CREATE_TYPES_TABLE, NULL_PARAMS).unwrap();
                for q in &POPULATE_TYPES_TABLE_QQ {
                    konn.execute(q, NULL_PARAMS).unwrap();
                }
                konn.execute(CREATE_UNION_TYPE_TABLE, NULL_PARAMS).unwrap();
                for q in &POPULATE_UNION_TYPE_TABLE_QQ {
                    konn.execute(q, NULL_PARAMS).unwrap();
                }
                konn.execute(CREATE_STRUCT_MEMBERS_TABLE, NULL_PARAMS).unwrap();
                konn.execute(CREATE_STATEMENT_TABLE, NULL_PARAMS).unwrap();
                Ok(())
            }
        }
    }

    fn connect(&mut self) -> Self::ConnectionResult {
        if let Ok(konn) = Connection::open(&self.path) {
            self.conn = Some(Box::new(konn));
        }
        Ok(())
    }

    // pub fn close(&mut self) -> Result<(), Box<dyn StdError>> {
    // pub fn disconnect(&mut self) -> Result<(), (Connection, Error)> {
    fn disconnect(&mut self) -> Self::ConnectionResult {
        if let Some(konn) = self.conn.take() {
            // konn.close()
            // let _ = konn.close();
            // ()
            konn.close()
        } else {
            Ok(())
        }
    }
}

    
#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use super::{TGStore, SqliteTGStore};

    #[test]
    fn create_store_test() {
        let mut path = temp_dir();
        path.push("tg-test");
        path.set_extension("db");
        let mut store = super::SqliteTGStore { path, conn: None };
        match store.connect() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
    #[test]
    fn setup_test() -> <SqliteTGStore as TGStore>::GeneralResult {
        let mut path = temp_dir();
        path.push("tg-test");
        path.set_extension("db");
        let mut store = super::SqliteTGStore { path, conn: None };
        store.connect();
        store.setup()
    }
}
