use std::fmt::format;

use crate::ast::BooleanType;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectType {
    NUMBER,
    BOOLEAN,
    STRING,
    NONE,
    FUNCTION,
    BUILTINFUNCTION,
    LIST,
    SET,
    HASH,
    ERROR,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Object {
    Num(Num),
    Bool(Bool),
    Str(Str),
    None(NoneLit),
}

impl Object {
    fn get_type(&self) -> ObjectType {
        match self {
            Self::Num(_) => ObjectType::NUMBER,
            Self::Bool(_) => ObjectType::BOOLEAN,
            Self::Str(_) => ObjectType::STRING,
            Self::None(_) => ObjectType::NONE,
        }
    }
}

pub trait Inspector {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Num {
    pub value: i64,
}

impl Inspector for Num {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Bool {
    pub value: BooleanType
}

impl Inspector for Bool {
    fn inspect(&self) -> String {
        match self.value {
            BooleanType::TRUE => "true".to_string(),
            BooleanType::FALSE => "false".to_string()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Str {
    pub value: i64,
}

impl Inspector for Str {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NoneLit;