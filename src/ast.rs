use std::{collections::{HashMap, HashSet}, hash::Hash, cmp::Ordering};

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
#[derive(Debug, Clone, Eq)]
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

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::FUNC(a), Expression::FUNC(b)) => {
                // Custom comparison logic for FUNC variant with HashMap args
                let keys1: HashSet<&Identifier> = a.args.keys().collect();
                let keys2: HashSet<&Identifier> = b.args.keys().collect();

                if keys1 != keys2 {
                    return false;
                }

                for key in keys1 {
                    if a.args[key] != b.args[key] {
                        return false;
                    }
                }

                true
            }
            (Expression::IDENTIFIER(a), Expression::IDENTIFIER(b)) => a == b,
            (Expression::NUMBERLITERAL(a), Expression::NUMBERLITERAL(b)) => a == b,
            (Expression::STRINGLITERAL(a), Expression::STRINGLITERAL(b)) => a == b,
            (Expression::PREFIX(a), Expression::PREFIX(b)) => a == b,
            (Expression::INFIX(a), Expression::INFIX(b)) => a == b,
            (Expression::BOOLEAN(a), Expression::BOOLEAN(b)) => a == b,
            (Expression::IF(a), Expression::IF(b)) => a == b,
            (Expression::WHILE(a), Expression::WHILE(b)) => a == b,
            (Expression::FOR(a), Expression::FOR(b)) => a == b,
            // Handle other variants here as needed
            _ => false,
        }
    }
}

impl Eq for BlockStatement {

}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}