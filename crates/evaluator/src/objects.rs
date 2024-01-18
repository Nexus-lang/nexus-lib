use lexer::tokens::Literal;
use parser::ast::{Ident, OptionallyTypedIdent, BlockStmt};

use crate::builtins::BuiltinFunc;

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
