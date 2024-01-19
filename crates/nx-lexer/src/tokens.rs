use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Use,
    Enum,
    Struct,
    Func,
    Var,
    Const,

    Loop,
    If,
    Else,
    When,

    And,
    Or,
    Operator(Operator),

    Break,
    Return,
    Local,

    Dot,
    Comma,
    Colon,
    QuestionMark,
    ExclamMark,
    Arrow,
    Assign,

    Literal(Literal),
    Ident(String),

    LParent,
    RParent,
    LSquare,
    RSquare,
    LCurly,
    RCurly,

    ConstAssign,
    VarAssign,

    Eol,
    Eof,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    Greater,
    Lesser,
    GreaterEquals,
    LesserEquals,
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Str(str) => str.to_string(),
                Literal::Num(num) => num.to_string(),
                Literal::Bool(bool) => bool.to_string(),
            }
        )
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Equals => "==",
                Operator::NotEquals => "!=",
                Operator::Greater => ">",
                Operator::Lesser => "<",
                Operator::GreaterEquals => ">=",
                Operator::LesserEquals => "<=",
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Asterisk => "*",
                Operator::Slash => "/",
            }
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit: String = match self {
            Token::Use => "use".into(),
            Token::Enum => "enum".into(),
            Token::Struct => "struct".into(),
            Token::Func => "func".into(),
            Token::Loop => "loop".into(),
            Token::If => "if".into(),
            Token::When => "when".into(),
            Token::Var => "var".into(),
            Token::Const => "const".into(),
            Token::And => "and".into(),
            Token::Or => "or".into(),
            Token::Break => "break".into(),
            Token::Return => "return".into(),
            Token::Local => "local".into(),
            Token::Literal(lit) => lit.to_string(),
            Token::Dot => ".".into(),
            Token::Comma => ",".into(),
            Token::Colon => ":".into(),
            Token::QuestionMark => "?".into(),
            Token::ExclamMark => "!".into(),
            Token::Eol => "Eol".into(),
            Token::Eof => "Eof".into(),
            Token::Operator(op) => op.to_string(),
            Token::Arrow => "->".into(),
            Token::LParent => "(".into(),
            Token::RParent => ")".into(),
            Token::LSquare => "[".into(),
            Token::RSquare => "]".into(),
            Token::LCurly => "{".into(),
            Token::RCurly => "}".into(),
            Token::Assign => "=".into(),
            Token::Else => "else".into(),
            Token::Ident(ident) => ident.into(),
            Token::ConstAssign => "::".into(),
            Token::VarAssign => ":=".into(),
        };
        write!(f, "{}", lit)
    }
}
