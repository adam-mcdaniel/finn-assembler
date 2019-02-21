use std::collections::HashMap;
use std::option::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Table<T> {
    contents: HashMap<String, T>
}


impl <T: Clone> Table<T> {
    pub fn new() -> Self {
        return Self{contents: HashMap::new()}
    }

    pub fn set(&mut self, name: String, value: T) {
        self.contents.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<T> {
        self.contents.get(&name).cloned()
    }

    pub fn keys(&self) -> Vec<String> {
        self.contents.iter().map(|(key, _)| key.clone()).collect()
    }

    pub fn values(&self) -> Vec<T> {
        self.contents.iter().map(|(_, value)| value.clone()).collect()
    }
}