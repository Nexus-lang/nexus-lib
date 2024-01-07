use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Var(VarStmt),
    Const(ConstStmt),
    Return(ReturnStmt),
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
    None,

    If(IfExpr),
    Loop(LoopExpr),
    When(WhenExpr),
    Func(FuncExpr),
    Call(CallExpr),
    List(ListExpr),
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

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

