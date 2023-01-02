#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TinyGraphError {
    desc: String,
}

impl fmt::Display for TinyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TinyGraph Error!!!!!!!!!!!!!!!!")
    }
}

impl Error for TinyGraphError {}

impl TinyGraphError {
    pub fn new(desc: String) -> Self {
        TinyGraphError { desc }
    }
    fn default() -> Self {
        TinyGraphError { desc: String::from("Ruh-roh!") }
    }
}

#[macro_export]
macro_rules! tg_error {
    ($($args:tt)*) => {
        Err(Box::new(TinyGraphError::new(format!($($args)*))))   
    }
}
