use std::collections::HashMap;

use crate::objects::Object;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Object>
}

impl Environment {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    pub fn set(&mut self, key: String, obj: Object) {
        self.store.insert(key, obj);
    }

    pub fn get(&mut self, key: String) -> Option<&Object> {
        self.store.get(&key)
    }
}