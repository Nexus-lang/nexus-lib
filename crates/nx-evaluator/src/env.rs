use std::collections::HashMap;

use crate::objects::Object;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, EnvObj>
}

#[derive(Debug)]
pub struct EnvObj {
    pub obj: Object,
    pub is_const: bool,
}

impl Environment {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    pub fn set(&mut self, key: String, obj: EnvObj) {
        self.store.insert(key, obj);
    }

    pub fn get(&mut self, key: String) -> Option<&EnvObj> {
        self.store.get(&key)
    }
}