use std::{collections::HashMap, fmt::Display, ops::Range};

#[derive(Debug, Clone, PartialEq)]
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

    // The vector is for the string literal as it appears in the source code
    // The hashmap is for args that need to be formatted
    String(Vec<char>, Option<HashMap<Range<usize>, Vec<Token>>>),
    Num(f64),
    Bool(bool),
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
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
            Token::Dot => ".".into(),
            Token::Comma => ",".into(),
            Token::Colon => ":".into(),
            Token::QuestionMark => "?".into(),
            Token::ExclamMark => "!".into(),
            Token::String(lit, _) => lit.iter().collect::<String>(),
            Token::Num(num) => num.to_string(),
            Token::Bool(bool) => bool.to_string(),
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
        })
    }
}
