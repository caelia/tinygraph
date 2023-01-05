use crate::tgconfig::Config;
use std::path::PathBuf;
use std::env::current_dir;

pub struct App {
    config: Config,    
}

impl App {
    pub fn new() -> Self {
        App { config: Config::new() }
    }
    pub fn config_get(&self, key: String) -> Option<&String> {
        self.config.data.get(&key)
    }
    pub fn default_dir(&self) -> Option<PathBuf> {
        let k = String::from("default_path");
        match self.config.data.get(&k) {
            Some(pathname) => Some(PathBuf::from(pathname)),
            None => None
        }
    }
    pub fn default_name(&self) -> String {
        let k = String::from("default_name");
        match self.config.data.get(&k) {
            Some(name) => name.to_string(),
            None => "tg_data.db".to_string(),
        }
    }
}
