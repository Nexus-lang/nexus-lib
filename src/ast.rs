use std::hash::Hash;

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
    FUNC(FuncExpression),
    CALL(CallExpression),
    LIST(ListExpression),
    INDEX(IndexExpression),
    ANNOTATION(AnnotationExpression),
    NONE(NoneLiteral),
    EMPTY,
}

/// All allowed booleans (true, false)
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum BooleanType {
    TRUE,
    FALSE,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum IfType {
    IF,
    ELSEIF,
    ELSE,
}

/// Operators for prefix and infix expressions
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum Operator {
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
pub struct Identifier {
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
    pub references: Vec<Identifier>,
}

// prefix (-, +, !)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct PrefixExpression {
    pub operator: Operator, // might want to create an enum for this
    pub right: Box<Expression>,
}

// Binary operations (4 + 5, 1 != 2...)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct IfExpression {
    pub if_type: IfType,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    // 0: condition; 1: consequence
    pub alternative: Option<Box<IfExpression>>,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FuncExpression {
    pub ident: Identifier,
    pub args: Vec<Identifier>,
    pub arg_types: Vec<Identifier>,
    pub return_type: Identifier,
    pub consequence: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct IndexExpression {
    pub list: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct ListExpression {
    pub content: Vec<Expression>,
    pub length: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct AnnotationExpression {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct NoneLiteral;

// booleans (true, false)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct Boolean {
    pub bool_type: BooleanType,
}
// ------------- end of all ast types

// the ast layout
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// This exists bc im too lazy to type this out all the time
impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
