use crate::{
    ast::BooleanType,
    builtin,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectType {
    NUMBER,
    BOOLEAN,
    STRING,
    NONE,
    RETURN,
    VAR,
    FUNCTION,
    BUILTINFUNCTION,
    LIST,
    SET,
    HASH,
    ERROR,
    UNMETIF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Object {
    Num(Num),
    Bool(Bool),
    Str(Str),
    None(NoneLit),
    Error(Error),
    UnMetIf(UnmetIf),
    Return(Return),
    Var(Var),
    BuiltInFunction(BuiltInFunction),
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
            Self::Var(_) => ObjectType::VAR,
            Self::UnMetIf(_) => ObjectType::UNMETIF,
            Self::BuiltInFunction(_) => ObjectType::BUILTINFUNCTION,
        }
    }
}

impl Literal for Object {
    fn literal(&self) -> String {
        match self {
            Object::Num(num) => (num.value as i32).to_string(),
            Object::Bool(bool) => format!("{:?}", bool.value).to_lowercase(),
            Object::Str(str) => str.value.to_string(),
            Object::None(_) => String::from("none"),
            Object::Error(_) => todo!(),
            Object::UnMetIf(_) => todo!(),
            Object::Return(ret) => format!("return {}", ret.value.literal()),
            Object::Var(var) => todo!(),
            Object::BuiltInFunction(func) => {
                let mut fmt_string = format!("{}(", func.func.name());
                func.args.iter().for_each(|x| {
                    fmt_string.push_str(x.literal().as_str());
                    fmt_string.push_str(", ")
                });
                fmt_string.push(')');
                fmt_string
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Num {
    pub value: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Bool {
    pub value: BooleanType,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Str {
    pub value: String,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct NoneLit;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            message: msg.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Return {
    pub value: Box<Object>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Var {
    pub value: Box<Object>,
}

// Kinda weird will be improved in rewrite I promise :3
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct UnmetIf;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BuiltInFunction {
    pub func: builtin::BuiltinFunction,
    pub args: Vec<Object>,
}

pub trait Literal {
    fn literal(&self) -> String;
}
