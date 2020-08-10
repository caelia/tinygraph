#![allow(dead_code)]

use rusqlite::{params, Connection, Result, Error};
use std::path::{Path, PathBuf};

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
    path: PathBuf,
    conn: Option<Box<Connection>>,
}

impl TGStore for SqliteTGStore {
    type GeneralResult = Result<(), Box<dyn std::error::Error>>;
    type ConnectionResult = Result<(), (Connection, Error)>;
    // type GeneralError = Box<dyn std::error::Error>;
    // type ConnectionError = (Error, Connection);
    fn new() -> Self {
        SqliteTGStore {
            path: PathBuf::new(),
            conn: None
        }
    }
    /*
    pub fn new(path: &PathBuf, overwrite: bool) -> Result<Self, Box<dyn std::error::Error>> {
        if &path.exists() & !overwrite {
            panic!("DB file '{:?}' already exists.", &path);
        }
        let konn = Connection::open(path)?;
        Ok(SqliteTGStore { path: *path, conn: Some(konn) })
    }
    */
    fn setup(&mut self) -> Self::GeneralResult {
        println!("SqliteTGStore::setup()");
        Ok(())
    }

    fn connect(&mut self) -> Self::ConnectionResult {
        if let Ok(konn) = Connection::open(&self.path) {
            self.conn = Some(Box::new(konn));
        }
        Ok(())
    }

    // pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
