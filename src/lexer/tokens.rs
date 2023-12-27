use std::{collections::HashMap, fmt::Display, ops::Range};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Use,
    Enum,
    Struct,
    Func,
    Loop,
    If,
    When,
    Var,
    Const,

    And,
    Or,

    Break,
    Return,

    Dot,
    Comma,
    Colon,
    QuestionMark,
    ExclamMark,

    // The vector is for the string literal as it appears in the source code
    // The hashmap is for args that need to be formatted
    String(Vec<char>, Option<HashMap<Range<usize>, String>>),
    Num(f64),
    Bool(bool),

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

    Comment(String),
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
            Token::Dot => ".".into(),
            Token::Comma => ",".into(),
            Token::Colon => ":".into(),
            Token::QuestionMark => "?".into(),
            Token::ExclamMark => "!".into(),
            Token::String(lit, _) => lit.iter().collect::<String>(),
            Token::Num(num) => num.to_string(),
            Token::Bool(bool) => bool.to_string(),
            Token::Comment(txt) => format!("# {}", txt),
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
        })
    }
}

impl Token {
    fn first_as_char(&self) -> char {
        let chars: Vec<char> = self.to_string().chars().collect();
        return if chars.len() > 0 {
            chars[0]
        } else {
            panic!("Cannot convert empty string to char")
        };
    }
}
