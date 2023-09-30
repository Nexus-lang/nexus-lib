use crate::{
    ast::{Arg, BlockStatement, BooleanType},
    builtins::{self, BuiltinType},
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
    RANGE,
    ERROR,
    UNMETIF,
    Type,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Type {
    NORMAL(TypeLit),
    BUILTIN(BuiltinType),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Object {
    Num(Num),
    Bool(Bool),
    Str(Str),
    None(NoneLit),
    Error(Error),
    UnMetExpr(UnmetExpr),
    Return(Return),
    Var(Var),
    Function(Function),
    BuiltInFunction(BuiltInFunction),
    Range(Range),
    Type(Type)
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
            Self::UnMetExpr(_) => ObjectType::UNMETIF,
            Self::BuiltInFunction(_) => ObjectType::BUILTINFUNCTION,
            Self::Function(_) => ObjectType::FUNCTION,
            Self::Range(_) => ObjectType::RANGE,
            Self::Type(_) => ObjectType::Type,
        }
    }
}

impl Literal for Object {
    fn literal(&self) -> String {
        match self {
            Object::Num(num) => num.value.to_string(),
            Object::Bool(bool) => format!("{:?}", bool.value).to_lowercase(),
            Object::Str(str) => str.value.to_string(),
            Object::None(_) => String::from("none"),
            Object::Error(_) => todo!(),
            Object::UnMetExpr(_) => todo!(),
            Object::Return(ret) => format!("return {}", ret.value.literal()),
            Object::Var(_) => todo!(),
            Object::BuiltInFunction(func) => {
                let mut fmt_string = format!("{}(", func.func.literal());
                func.args.iter().for_each(|x| {
                    fmt_string.push_str(x.literal().as_str());
                    fmt_string.push_str(", ")
                });
                fmt_string.push(')');
                fmt_string
            }
            Object::Function(_) => todo!(),
            Object::Range(range) => format!("{}..{}", range.left.literal(), range.right.literal()),
            Object::Type(type_type) => match &type_type {
                Type::NORMAL(normal) => normal.type_name.to_string(),
                Type::BUILTIN(builtin) => builtin.literal(),
            },
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Num {
    pub value: f64,
}

impl Num {
    pub fn new(value: f64) -> Self {
        Num { value }
    }
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
    pub fn new(msg: String) -> Error {
        Error {
            message: msg,
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
    // It's called local. why is it public then xD
    pub is_local: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Function {
    pub args: Vec<Arg>,
    pub body: BlockStatement,
}

impl Function {
    pub fn empty() -> Self {
        Function {
            args: Vec::new(),
            body: BlockStatement {
                statements: Vec::new(),
            },
        }
    }
}

// Kinda weird will be improved in rewrite I promise :3
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct UnmetExpr;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BuiltInFunction {
    pub func: builtins::BuiltinFunction,
    pub args: Vec<Object>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Range {
    pub left: Box<Object>,
    pub right: Box<Object>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TypeLit {
    type_name: String,
    is_builtin: bool,
}

pub trait Literal {
    fn literal(&self) -> String;
}
