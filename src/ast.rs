#[derive(PartialEq, PartialOrd, Debug)]
pub enum Statement {
    VAR(VarStatement),
    RETURN(ReturnStatement),
    EXPRESSION(ExpressionStatement),
    EMPTY,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    IDENTIFIER(Identifier),
    NUMBERLITERAL(NumberLiteral),
    STRINGLITERAL(StringLiteral),
    PREFIX(PrefixExpression),
    EMPTY,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Identifier {
    pub value: String
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct NumberLiteral {
    pub value: i64,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct PrefixExpression {
    pub operator: String, // might want to create an enum for this
    pub right: Box<Expression>,
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