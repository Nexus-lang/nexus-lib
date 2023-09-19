use std::collections::HashMap;

use crate::object::{Object, NoneLit, Str};

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let s = HashMap::new();
        Environment { store: s}
    }

    pub fn get(&self, name: String) -> Result<Object, ()> {
        let obj = match self.store.get(&name) {
            Some(val) => Ok(val.clone()),
            None => {
                Err(()) // Return an error if the key is not found
            }
        };
        obj
    }

    pub fn set(&mut self, name: &String, val: &Object) -> Object {
        self.store.insert(name.to_string(), val.clone());
        val.clone()
    }
}
