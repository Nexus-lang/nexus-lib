/// Statements, Code parts that don't give back a value
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Statement {
    VAR(VarStatement),
    CONST(ConstStatement),
    RETURN(ReturnStatement),
    LOCAL(LocalStatement),
    /// Statement wrapper for expressions
    ///
    /// Required because The ast only
    /// accepts a list of statements
    EXPRESSION(ExpressionStatement),
    EMPTY,
}

/// Expressions, Code parts that don't give back a value
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expression {
    IDENTIFIER(Identifier),
    NUMBERLITERAL(NumberLiteral),
    STRINGLITERAL(StringLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    BOOLEAN(Boolean),
    IF(IfExpression),
    AS(AsExpression),
    EMPTY,
}

/// All allowed booleans (true, false)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Booleans {
    TRUE,
    FALSE,
}

/// Operators for prefix and infix expressions
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Operators {
    PLUS,         // both
    MINUS,        // both
    BANG,         // pre
    MULTIPLY,     // in
    DIVIDE,       // in
    EQUAL,        // in
    NOTEQUAL,     // in
    GREATTHAN,    // in
    LESSTHAN,     // in
    GREATOREQUAL, // in
    LESSOREQUAL,  // in
    AS,           // in
    ILLEGAL,
}

// All ast types
// ident
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct LocalStatement {
    pub left: Box<Statement>,
}

// var
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Expression,
}

// const
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ConstStatement {
    pub name: Identifier,
    pub value: Expression,
}

// return
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

// expression (literals, arithmetic operations, functions, if, while...)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

/// Code block enclosed by curly brackets
///  ```
/// if true {
///     print("Hello, World!")
/// }
///  ```
/// 
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

// number literal
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct NumberLiteral {
    pub value: i64,
}

// string literal
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

// prefix (-, +, !)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct PrefixExpression {
    pub operator: Operators, // might want to create an enum for this
    pub right: Box<Expression>,
}
// Binary operations (4 + 5, 1 != 2...)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operators,
}

// TODO: Implement this
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct AsExpression {
    pub from: Box<Expression>,
    pub to: Box<Expression>,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: BlockStatement,
}

// booleans (true, false)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Boolean {
    pub bool_type: Booleans,
}
// ------------- end of all ast types

// the ast layout
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
