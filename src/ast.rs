use crate::tokens::Token;

#[derive(PartialEq)]
pub enum Statement {
    VAR(VarStatement),
    EMPTY,
}

#[derive(PartialEq, PartialOrd)]
pub enum Expression {
    EMPTY,
}

#[derive(PartialEq, PartialOrd)]
pub struct Identifier {
    pub value: String
}

#[derive(PartialEq, PartialOrd)]
pub struct VarStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements: statements }
    }
}