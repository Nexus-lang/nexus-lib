#[derive(PartialEq, PartialOrd, Debug)]
pub enum Statement {
    VAR(VarStatement),
    RETURN(ReturnStatement),
    EXPRESSION(ExpressionStatement),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    IDENTIFIER(Identifier),
    NUMBERLITERAL(NumberLiteral)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Identifier {
    pub value: String
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ReturnStatement {
    pub return_value: Option<Expression>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct NumberLiteral {
    pub value: i64
}

// The node
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements: statements }
    }
}