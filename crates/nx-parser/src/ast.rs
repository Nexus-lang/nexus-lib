use std::fmt::Display;

use nx_lexer::tokens::Literal;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Variable(VarStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Local(LocalStmt),
    Use(UseStmt),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal),
    Prefix(PrefixExpr),
    Infix(InfixExpr),
    Index(IndexExpr),
    Call(CallExpr),
    List(ListExpr),
    None,

    If(IfExpr),
    Loop(LoopExpr),
    When(WhenExpr),
    Func(FuncExpr),
    Annotation(AnnotationExpr),
    Struct(StructExpr),
    Enum(EnumExpr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct OptionallyTypedIdent {
    pub ident: Ident,
    pub _type: Option<Ident>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarStmt {
    pub name: OptionallyTypedIdent,
    pub val: Expression,
    pub is_const: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub val: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStmt {
    pub label: Option<Ident>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalStmt {
    pub val: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseStmt {
    // TODO: Import paths
    pub import: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpr {
    pub op: PrefixOp,
    pub val: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpr {
    pub op: InfixOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexExpr {
    pub list: Box<Expression>,
    pub pos: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    // needs to be an expression
    // because of weird infix parsing
    pub ident: Box<Expression>,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListExpr {
    pub list: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    pub _type: IfType,
    pub cond: Option<Box<Expression>>,
    pub block: BlockStmt,
    pub alt: Option<Box<IfExpr>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopExpr {
    pub _type: LoopType,
    pub cond: Option<Box<Expression>>,
    pub block: BlockStmt,
    pub alt: Option<Box<LoopExpr>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhenExpr {
    pub comp_val: Option<Box<Expression>>,
    pub cases: Vec<CaseStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncExpr {
    pub ret_type: Option<Ident>,
    pub args: Vec<OptionallyTypedIdent>,
    pub block: BlockStmt,
}

// TODO: Finish this
#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationExpr {
    pub name: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructExpr {
    pub fields: Vec<OptionallyTypedIdent>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumExpr {
    pub consts: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseStmt {
    pub _type: CaseType,
    /// This should be Some(...) if the _type is Regular and
    /// the WhenExpr.comp_val is Some(...) as well
    pub comp_cond: Option<Box<Expression>>,
    /// This should be None unless the WhenExpr.comp_val is None
    pub comp_val: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrefixOp {
    Pos,
    Neg,
    Not,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NEq,
    GT,
    LT,
    GTEq,
    LTEq,
    As,
    In,
    Range,
    Assign,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IfType {
    If,
    Else,
    ElseIf,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LoopType {
    For,
    While,
    ElseFor,
    ElseWhile,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CaseType {
    Regular,
    Else,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Statement::Variable(var) => format!(
                    "{} {} = {}",
                    match var.is_const {
                        true => "const",
                        false => "var",
                    },
                    todo!(), //var.name,
                    var.val,
                ),
                Statement::Return(ret) => todo!(),
                Statement::Break(br) => todo!(),
                Statement::Local(lcl) => todo!(),
                Statement::Use(_use) => todo!(),
                Statement::Expression(expr) => expr.to_string(),
            }
        )
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Ident(ident) => ident.to_string(),
                Expression::Literal(lit) => lit.to_string(),
                Expression::Prefix(prefix) => prefix.to_string(),
                Expression::Infix(infix) => infix.to_string(),
                Expression::Index(_) => todo!(),
                //Expression::Call(call) => call.to_string(),
                Expression::List(_) => todo!(),
                Expression::None => "none".into(),
                //Expression::If(_if) => _if.to_string(),
                //Expression::Loop(_loop) => _loop.to_string(),
                Expression::When(_) => todo!(),
                //Expression::Func(func) => func.to_string(),
                Expression::Annotation(_) => todo!(),
                Expression::Struct(_) => todo!(),
                Expression::Enum(_) => todo!(),
                _ => todo!(),
            }
        )
    }
}

impl Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.op {
                PrefixOp::Pos => "+",
                PrefixOp::Neg => "-",
                PrefixOp::Not => "!",
            },
            self.to_string()
        )
    }
}

impl Display for InfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.left.to_string(),
            match self.op {
                InfixOp::Add => "+",
                InfixOp::Sub => "-",
                InfixOp::Mul => "*",
                InfixOp::Div => "/",
                InfixOp::Eq => "==",
                InfixOp::NEq => "!=",
                InfixOp::GT => ">",
                InfixOp::LT => "<",
                InfixOp::GTEq => ">=",
                InfixOp::LTEq => "<=",
                InfixOp::As => "as",
                InfixOp::In => "in",
                InfixOp::Range =>
                    return write!(f, "{}..{}", self.left.to_string(), self.right.to_string()),
                InfixOp::Assign => "=",
            },
            self.right.to_string()
        )
    }
}
