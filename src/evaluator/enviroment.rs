use std::collections::HashMap;

use super::object::Object;

type OptionalEnvObj = Option<Box<EnvObj>>;

#[derive(Debug, Clone)]
pub enum EnvObj {
    OBJ(Obj),
    SCOPE(Scope),
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub obj: Box<Object>,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct Scope {
    objs: HashMap<String, OptionalEnvObj>,
}

impl Scope {
    pub fn set(&mut self, name: &String, obj: Obj) {
        match self.objs.insert(format!("$var_{}", name), Some(Box::from(EnvObj::OBJ(obj)))) {
            Some(_) => (),
            None => (),
        }
    }

    pub fn get(&mut self, name: &String) -> Result<Obj, ()> {
        match self.objs.get(&format!("$var_{}", name)) {
            Some(obj) => match obj {
                Some(val) => match &**val {
                    EnvObj::OBJ(val) => Ok(val.to_owned()),
                    EnvObj::SCOPE(_) => todo!(),
                },
                None => todo!(),
            },
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    scope_id: i64,
    objs: HashMap<String, EnvObj>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scope_id: 0,
            objs: HashMap::new(),
        }
    }

    pub fn set_global(&mut self, name: &String, obj: Obj) {
        match self.objs.insert(format!("$var_{}", name), EnvObj::OBJ(obj)) {
            Some(_) => (),
            None => (),
        }
    }

    pub fn get_global(&self, name: &String) -> Result<Obj, ()> {
        match self.objs.get(&format!("$var_{}", name)) {
            Some(obj) => match &obj {
                EnvObj::OBJ(val) => Ok(val.to_owned()),
                EnvObj::SCOPE(_) => todo!(),
            },
            None => Err(()),
        }
    }

    pub fn modify_global(&mut self, name: &String, new_val: Object) {
        match self.objs.get_mut(&format!("$var_{}", name)) {
            Some(val) => match val {
                EnvObj::OBJ(obj) => if !obj.is_const {
                    obj.obj = Box::new(new_val)
                } else {
                    todo!("Cannot modify a constant variable")
                }
                EnvObj::SCOPE(_) => todo!(),
            },
            None => todo!(),
        }
    }

    pub fn create_scope(&mut self) -> Scope {
        self.scope_id += 1;
        match self.objs.insert(
            format!("#scope_{}", self.scope_id),
            EnvObj::SCOPE(Scope { objs: HashMap::new() }),
        ) {
            Some(_) => Scope { objs: HashMap::new() },
            None => Scope { objs: HashMap::new() },
        }
    }
}
