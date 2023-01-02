use std::path::PathBuf;
use std::collections::HashMap;

pub struct Config {
    pub data: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config { data: HashMap::new() }
    }
}
