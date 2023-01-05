use std::path::PathBuf;
use std::collections::HashMap;

pub struct Config {
    pub data: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config { data: HashMap::new() }
    }
    pub fn get(&self, key: String) -> Option<String> {
        Some(String::from(":XXX:"))
    }
    pub fn set(&mut self, key: String, value: String) {
        let _ = self.data.insert(key, value);
    }
}
