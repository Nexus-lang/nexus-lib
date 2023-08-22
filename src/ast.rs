/// Statements Code parts that don't give back a value
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Statement {
    VAR(VarStatement),
    RETURN(ReturnStatement),
    /// Statement wrapper for expressions
    /// 
    /// Required because The ast only
    /// accepts a list of statements
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

// ident
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Identifier {
    pub value: String
}

// var
#[derive(PartialEq, PartialOrd, Debug)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Expression,
}

// return
#[derive(PartialEq, PartialOrd, Debug)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

// expression (literals, arithmetic operations, functions, if, while...)
#[derive(PartialEq, PartialOrd, Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

// number literal
#[derive(PartialEq, PartialOrd, Debug)]
pub struct NumberLiteral {
    pub value: i64,
}

// string literal
#[derive(PartialEq, PartialOrd, Debug)]
pub struct StringLiteral {
    pub value: String,
}

// prefix (-, +, !)
#[derive(PartialEq, PartialOrd, Debug)]
pub struct PrefixExpression {
    pub operator: String, // might want to create an enum for this
    pub right: Box<Expression>,
}

// the ast layout
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
