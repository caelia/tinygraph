use crate::tgconfig::Config;
use crate::tg_error;
use std::path::PathBuf;
use std::env::current_dir;

pub trait App {
    fn new() -> Self;
    fn default_dir(&self) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let k = String::from("default_path");
        match self.config_get(k) {
            Some(pathname) => Ok(Some(PathBuf::from(pathname))),
            None => Ok(None)
        }
    }
    fn default_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let k = String::from("default_name");
        match self.config_get(k) {
            Some(name) => Ok(name.to_string()),
            None => Ok("tg_data.db".to_string()),
        }
    }
    fn config_get(&self, key: String) -> Option<String>;
    fn config_set(&mut self, key: String, value: String);
}
