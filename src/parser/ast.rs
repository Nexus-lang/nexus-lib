use std::hash::Hash;

/// Statements, Code parts that don't give back a value
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Statement {
    VAR(VarStatement),
    CONST(ConstStatement),
    RETURN(ReturnStatement),
    LOCAL(LocalStatement),
    USE(UseStatement),
    /// Statement wrapper for expressions
    ///
    /// Required because The ast only
    /// accepts a list of statements
    EXPRESSION(ExpressionStatement),
    EMPTY,
}

/// Expressions, Code parts that don't give back a value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    IDENTIFIER(Identifier),
    NUMBERLITERAL(NumberLiteral),
    STRINGLITERAL(StringLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    BOOLEAN(Boolean),
    IF(IfExpression),
    LOOP(LoopExpression),
    WHEN(WhenExpression),
    FUNC(FuncExpression),
    CALL(CallExpression),
    LIST(ListExpression),
    INDEX(IndexExpression),
    ANNOTATION(AnnotationExpression),
    NONE(NoneLiteral),
    EMPTY, // FIXME: this is deprecated. migrate everything to options
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum IfType {
    IF,
    ELSEIF,
    ELSE,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum LoopType {
    WHILE,
    FOR,
}

/// Operators for prefix and infix expressions
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
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
    IN,           // in
    RANGE,        // in
    ASSIGN,       // assign
    ILLEGAL,
}

// All ast types
// ident
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Hash)]
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
    pub var_type: Type,
    pub value: Expression,
}

// const
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ConstStatement {
    pub name: Identifier,
    pub const_type: Type,
    pub value: Expression,
}

// return
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct UseStatement {
    pub path: String,
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

impl BlockStatement {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self { statements: stmts }
    }

    pub fn new_from_single(stmt: Statement) -> Self {
        Self::new(vec![stmt])
    }
}

// number literal
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct NumberLiteral {
    pub value: f64,
}

// string literal
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub references: Vec<Expression>,
}

// prefix (-, +, !)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct PrefixExpression {
    pub operator: Operator, // might want to create an enum for this
    pub right: Box<Expression>,
}

// Binary operations (4 + 5, 1 != 2...)
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct IfExpression {
    pub if_type: IfType,
    pub condition: Option<Box<Expression>>,
    pub consequence: BlockStatement,
    // 0: condition; 1: consequence
    pub alternative: Option<Box<IfExpression>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WhenExpression {
    pub value: Box<Expression>,
    pub cases: Vec<CaseStatement>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CaseType {
    DEFAULT,
    ELSE,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CaseStatement {
    pub _type: CaseType,
    pub case_condition: Option<Expression>, // Option cuz it can also be else
    pub case_consequence: BlockStatement,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct LoopExpression {
    pub loop_type: LoopType,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FuncExpression {
    pub args: Vec<Arg>,
    pub return_type: Type,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Arg {
    pub arg: Identifier,
    pub arg_type: Type
}

pub type Type = Option<Identifier>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IndexExpression {
    pub list: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ListExpression {
    pub content: Vec<Expression>,
    pub length: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AnnotationExpression {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct NoneLiteral;

// booleans (true, false)
#[derive(PartialEq, PartialOrd, Debug, Clone, Eq)]
pub struct Boolean {
    pub bool_type: bool,
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
