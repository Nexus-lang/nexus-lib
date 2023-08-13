use crate::tokens::Token;

#[derive(PartialEq, Debug)]
pub enum Statement {
    VAR(VarStatement),
    RETURN(ReturnStatement),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
}

#[derive(PartialEq, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

#[derive(PartialEq, Debug)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement {
    pub return_value: Option<Expression>,
}

// The node
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements: statements }
    }
}