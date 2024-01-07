use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Var(VarStmt),
    Const(ConstStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Local(LocalStmt),
    Use(UseStmt),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq)]
pub struct OptionallyTypedIdent {
    pub ident: Ident,
    pub _type: Option<Ident>,
}

#[derive(Debug, PartialEq)]
pub struct VarStmt {
    pub name: OptionallyTypedIdent,
    pub val: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ConstStmt {
    pub name: OptionallyTypedIdent,
    pub val: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
    pub val: Ident,
}

#[derive(Debug, PartialEq)]
pub struct BreakStmt {
    pub ret_val: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq)]
pub struct LocalStmt {
    pub val: Box<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct UseStmt {
    // TODO: Import paths
    pub import: Ident,
}

#[derive(Debug, PartialEq)]
pub struct PrefixExpr {
    pub op: PrefixOp,
    pub val: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct InfixExpr {
    pub op: InfixOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IndexExpr {
    pub list: Box<Expression>,
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    pub ident: Ident,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ListExpr {
    pub list: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IfExpr {
    pub _type: IfType,
    pub cond: Box<Expression>,
    pub block: BlockStmt,
    pub alt: Option<Box<IfExpr>>
}

#[derive(Debug, PartialEq)]
pub struct LoopExpr {
    pub _type: LoopType,
    pub cond: Option<Box<Expression>>,
    pub block: BlockStmt,
    pub alt: Option<Box<LoopExpr>>
}

#[derive(Debug, PartialEq)]
pub struct WhenExpr {
    pub comp_val: Option<Box<Expression>>,
    pub cases: Vec<CaseStmt>,
}

#[derive(Debug, PartialEq)]
pub struct FuncExpr {
    pub ident: OptionallyTypedIdent,
    pub args: Vec<OptionallyTypedIdent>,
    pub block: BlockStmt,
}

// TODO: Finish this
#[derive(Debug, PartialEq)]
pub struct AnnotationExpr {
    pub name: Ident,
}

#[derive(Debug, PartialEq)]
pub struct StructExpr {
    pub fields: Vec<OptionallyTypedIdent>,
}

#[derive(Debug, PartialEq)]
pub struct EnumExpr {
    pub consts: Ident,
}

#[derive(Debug, PartialEq)]
pub struct BlockStmt {
    stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct CaseStmt {
    pub _type: CaseType,
    /// This should be Some(...) if the _type is Regular and
    /// the WhenExpr.comp_val is Some(...) as well
    pub comp_cond: Option<Box<Expression>>,
    /// This should be None unless the WhenExpr.comp_val is None
    pub comp_val: Option<Box<Expression>>
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Num(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum PrefixOp {
    Pos,
    Neg,
    Not,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum IfType {
    If,
    Else,
    ElseIf,
}

#[derive(Debug, PartialEq)]
pub enum LoopType {
    For,
    While,
    ElseFor,
    ElseWhile,
    Else,
}

#[derive(Debug, PartialEq)]
pub enum CaseType {
    Regular,
    Else,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

