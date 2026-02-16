use directories::ProjectDirs;

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

impl Default for Config {
    fn default() -> Self {
        let mut data = HashMap::new();
        let data_dir = match ProjectDirs::from("org", "tinygraph", "tinygraph") {
            Some(pdirs) => pdirs.data_dir().to_string_lossy().to_string(),
            None => ".".to_string(),
        };
        data.insert("default_directory".to_string(), data_dir);
        data.insert("default_name".to_string(), "tg_data.db".to_string());
        Self { data }
    }
}
