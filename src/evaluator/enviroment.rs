use std::collections::HashMap;

use crate::evaluator::object::Object;

#[derive(Debug, Clone)]
pub struct EnvObj {
    pub is_const: bool,
    pub is_local: bool,
    pub obj: Object,
}

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, EnvObj>,
}

impl Environment {
    pub fn new() -> Self {
        let s = HashMap::new();
        Environment { store: s}
    }

    pub fn get(&self, name: &String) -> Result<EnvObj, ()> {
        let obj = match self.store.get(name) {
            Some(val) => Ok(val.clone()),
            None => {
                Err(()) // Return an error if the key is not found
            }
        };
        obj
    }

    pub fn set(&mut self, name: &String, val: &Object, is_local: bool, is_const: bool) -> Object {
        self.store.insert(name.to_string(), EnvObj { is_const, is_local, obj: val.to_owned() });
        val.clone()
    }

    pub fn modify(&mut self, name: &String, new_val: Object) {
        match self.store.get_mut(name) {
            Some(val) => if !val.is_const {
                val.obj = new_val
            } else {
                todo!("Cannot modify a constant variable")
            },
            None => todo!(),
        }
    }
}