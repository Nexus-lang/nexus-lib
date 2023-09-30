use crate::{
    ast::{BlockStatement, BooleanType, Identifier, Arg},
    builtins,
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
            Object::UnMetExpr(_) => todo!(),
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
            Object::Function(func) => todo!(),
            Object::Range(range) => format!("{}..{}", range.left.literal(), range.right.literal())
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

pub trait Literal {
    fn literal(&self) -> String;
}
