use std::
    fmt::Display
;

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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    Bool(bool),
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
            Token::Eol => "\n".into(),
            Token::Eof => "Eol".into(),
            Token::Equals => "==".into(),
            Token::NotEquals => "!=".into(),
            Token::Greater => ">".into(),
            Token::Lesser => "<".into(),
            Token::GreaterEquals => ">=".into(),
            Token::LesserEquals => "<=".into(),
            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::Asterisk => "*".into(),
            Token::Slash => "/".into(),
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
