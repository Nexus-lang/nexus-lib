use crate::tokens::Token;

#[derive(PartialEq, Debug)]
pub enum Statement {
    VAR(VarStatement)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    EMPTY,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements: statements }
    }
}