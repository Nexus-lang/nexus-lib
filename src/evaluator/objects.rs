use std::fmt::Display;

use crate::{
    evaluator::builtins::BuiltinFunc,
    lexer::tokens::Literal,
    parser::ast::{BlockStmt, Ident, OptionallyTypedIdent},
};

use super::util;

#[derive(Debug, Clone)]
pub enum Object {
    Lit(Literal),
    None,
    // TODO: Implement error system
    Err,
    // TOOD: Implement multi file shenanigans
    Use,
    // Rc<Object> is the return value
    Ret(Box<Object>),
    // Ident is the label
    Br(Ident),
    Func(FuncObj),
    BuiltinFunc(BuiltinFunc),
    // TODO: Implement these
    Range,
    Type,
    List,
}

#[derive(Debug, Clone)]
pub struct FuncObj {
    pub args: Vec<OptionallyTypedIdent>,
    pub block: BlockStmt,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Comparable {
    Lit(Literal),
    None,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Lit(lit) => lit.to_string(),
                Object::None => "none".into(),
                Object::Err => todo!(),
                Object::Use => todo!(),
                Object::Ret(_) => todo!(),
                Object::Br(_) => todo!(),
                Object::Func(func) => format!(
                    "func({}) {{\n{}\n}}",
                    util::typed_vec_to_string(&func.args),
                    util::block_to_string(&func.block)
                ),
                Object::BuiltinFunc(func) => match func.get_ret_val() {
                    Some(func) => func.to_string(),
                    None => todo!(),
                },
                Object::Range => todo!(),
                Object::Type => todo!(),
                Object::List => todo!(),
            }
        )
    }
}
