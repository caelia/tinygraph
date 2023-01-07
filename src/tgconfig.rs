use std::path::PathBuf;
use std::collections::HashMap;

pub trait Config: Default {
    type KeyType;
    type ValueType;

    fn new() -> Self;
    fn get(&self, key: Self::KeyType) -> Result<Option<&Self::ValueType>, Box<dyn std::error::Error>>;
    fn set(&mut self, key: Self::KeyType, value: Self::ValueType) -> Result<(), Box<dyn std::error::Error>>;
}
