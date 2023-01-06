#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod cli;
// pub mod query;
pub mod app;
pub mod tgconfig;
#[macro_use]
pub mod error;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use rusqlite::{params, Connection};
use rusqlite::Result as RsqResult;
use rusqlite::Error as RsqError;
use rusqlite::OpenFlags as RsqOpenFlags;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use error::*;

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

/*
pub trait DbOptions<T> {
    fn new(opts: Vec<(String, T)>) -> Self;
    fn get(&self, key: String) -> Result<Option<T>, Box<dyn std::error::Error>>; 
    fn set(&mut self, key: String, value: T) -> Result<(), Box<dyn std::error::Error>>;
}
*/

pub trait Database {
    fn new(dir: PathBuf, filename: String, init: bool, replace: bool, 
           options: Vec<(String, String)>)
           -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn open(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    // fn connect(&mut self) { }
    fn close(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

    
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
