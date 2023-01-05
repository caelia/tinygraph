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
        match self.data.get(&key) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        let _ = self.data.insert(key, value);
    }
}
