#![allow(dead_code)]

use rusqlite::{params, Connection, Result};
use std::path::{Path, PathBuf};


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
pub struct Database<'a> {
    path: &'a Path,
    conn: Option<Connection>,
}

impl<'a> Database<'a> {
    pub fn new(path: &'a Path, overwrite: bool) -> Result<Self, Box<dyn std::error::Error>> {
        if &path.exists() & !overwrite {
            panic!("DB file '{:?}' already exists.", &path);
        }
        let konn = Connection::open(&path)?;
        Ok(Database { path: &path, conn: Some(konn) })
    }

    pub fn open(&mut self) {
        if let Ok(konn) = Connection::open(self.path) {
            self.conn = Some(konn);
        }
    }

    pub fn close(&mut self) {
        if let Some(konn) = &self.conn {
            // Doesn't verking!
            if let Ok(_) = &konn.close() {
                self.conn = None
            }
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
