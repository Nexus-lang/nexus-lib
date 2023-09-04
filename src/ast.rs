use std::{collections::HashMap, hash::Hash};

/// Statements, Code parts that don't give back a value
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Expression {
    IDENTIFIER(Identifier),
    NUMBERLITERAL(NumberLiteral),
    STRINGLITERAL(StringLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    BOOLEAN(Boolean),
    IF(IfExpression),
    WHILE(WhileExpression),
    FOR(ForExpression),
    EMPTY,
}

/// All allowed booleans (true, false)
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum Booleans {
    TRUE,
    FALSE,
}

/// Operators for prefix and infix expressions
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
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
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Hash)]
pub struct  Identifier {
    pub value: String,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct LocalStatement {
    pub left: Box<Statement>,
}

// var
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct VarStatement {
    pub name: Identifier,
    pub value: Expression,
}

// const
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct ConstStatement {
    pub name: Identifier,
    pub value: Expression,
}

// return
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

// expression (literals, arithmetic operations, functions, if, while...)
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
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
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

// number literal
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct NumberLiteral {
    pub value: i64,
}

// string literal
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct StringLiteral {
    pub value: String,
}

// prefix (-, +, !)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct PrefixExpression {
    pub operator: Operators, // might want to create an enum for this
    pub right: Box<Expression>,
}
// Binary operations (4 + 5, 1 != 2...)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operators,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: BlockStatement,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct ForExpression {
    pub ident: Identifier,
    pub loop_list: Box<Expression>,
    pub consequence: BlockStatement,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct WhileExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncExpression {
    pub ident: Identifier,
    pub args: HashMap<Identifier, Identifier>,
    pub return_val: Identifier,
    pub consequence: BlockStatement,
}

// booleans (true, false)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct Boolean {
    pub bool_type: Booleans,
}
// ------------- end of all ast types

// the ast layout
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
