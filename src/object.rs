use crate::ast::BooleanType;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectType {
    NUMBER,
    BOOLEAN,
    STRING,
    NONE,
    RETURN,
    FUNCTION,
    BUILTINFUNCTION,
    LIST,
    SET,
    HASH,
    ERROR,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Object {
    Num(Num),
    Bool(Bool),
    Str(Str),
    None(NoneLit),
    Error(Error),
    Return(Return),
}

impl Object {
    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Num(_) => ObjectType::NUMBER,
            Self::Bool(_) => ObjectType::BOOLEAN,
            Self::Str(_) => ObjectType::STRING,
            Self::None(_) => ObjectType::NONE,
            Self::Error(_) => ObjectType::ERROR,
            Self::Return(_) => ObjectType::RETURN,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Num {
    pub value: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Bool {
    pub value: BooleanType
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Str {
    pub value: i64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct NoneLit;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error { message: format!("Error: {}", msg) }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Return {
    pub value: Box<Object>,
}
