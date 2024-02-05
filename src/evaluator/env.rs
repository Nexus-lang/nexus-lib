use std::{collections::HashMap, ops::{Deref, DerefMut}};

use super::objects::Object;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, EnvObj>,
}

#[derive(Debug)]
pub struct EnvObj {
    pub obj: Object,
    pub is_const: bool,
}

impl EnvObj {
    pub fn new(obj: Object, is_const: bool) -> Self {
        Self { obj, is_const }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, obj: EnvObj) {
        self.store.insert(key, obj);
    }

    pub fn get(&self, key: &String) -> Option<&EnvObj> {
        self.store.get(key)
    }
}
