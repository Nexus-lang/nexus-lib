#[derive(Debug)]
#[allow(dead_code)]
#[allow(macro_use_extern_crate)]
pub enum TokenTypes {
    // Keywords
    VAR,
    FUN,
    FOR,
    WHILE,
    IF,
    ELSE,

    // Special character
    LCURLY,       // {
    RCURLY,       // }
    LPARENT,     // (
    RPARENT,     // )
    LSQUAREBRAC, // [
    RSQUAREBRAC, // ]
    COMMENT,     // //
    QUOTMARKS,   // ""
    COMMA,       // ,
    COLON,       // :

    // Identifier, literals
    IDENT,
    NUMBER,
    FALSE,
    TRUE,

    // Other
    ILLEGAL, // illeagl expression
    EOL,     // End of line

    // Arithmetic operators and alike
    PLUS,        // +
    MINUS,       // -
    DIVIDE,      // /
    MULTIPLY,    // *
    ASSIGN,      // =

    EQUAL,       // ==
    NOTEQUAL,    // !=
    GREATERTHAN, // >
    LESSERTHAN,    // <
    GREATEROREQUALTHAN, // >=
    LESSEROREQUALTHAN,    // <=

    // Structures
    STRUCT,
    ENUM,
}

impl TokenTypes {
    pub fn literal(&self) -> String {
        match self {
            TokenTypes::VAR => {String::from("var")}
            TokenTypes::FUN => {String::from("fun")}
            TokenTypes::FOR => {String::from("for")}
            TokenTypes::WHILE => {String::from("while")}
            TokenTypes::IF => {String::from("if")}
            TokenTypes::ELSE => {String::from("else")}

            TokenTypes::IDENT => {String::from("IDENT")}
            TokenTypes::NUMBER => {String::from("NUMBER")}
            TokenTypes::TRUE => {String::from("true")}
            TokenTypes::FALSE => {String::from("false")}

            TokenTypes::EOL => {String::from("EOL")}
            TokenTypes::ILLEGAL => {String::from("ILLEGAL")}

            TokenTypes::ASSIGN => {String::from("=")}
            TokenTypes::PLUS => {String::from("+")}
            TokenTypes::MINUS => {String::from("-")}
            TokenTypes::MULTIPLY => {String::from("*")}
            TokenTypes::DIVIDE => {String::from("/")}
            
            TokenTypes::EQUAL => {String::from("==")}
            TokenTypes::NOTEQUAL => {String::from("!=")}
            TokenTypes::GREATERTHAN => {String::from(">")}
            TokenTypes::LESSERTHAN => {String::from("<")}
            TokenTypes::GREATEROREQUALTHAN => {String::from(">=")}
            TokenTypes::LESSEROREQUALTHAN => {String::from("<=")}

            TokenTypes::COMMENT => {String::from("//")}
            
            TokenTypes::COLON => {String::from(":")}
            TokenTypes::COMMA => {String::from(",")}
            TokenTypes::LCURLY => {String::from("{")}
            TokenTypes::RCURLY => {String::from("}")}
            TokenTypes::LPARENT => {String::from("(")}
            TokenTypes::RPARENT => {String::from(")")}
            TokenTypes::LSQUAREBRAC => {String::from("[")}
            TokenTypes::RSQUAREBRAC => {String::from("]")}
            TokenTypes::QUOTMARKS => {String::from("\"")}

            TokenTypes::STRUCT => {String::from("struct")}
            TokenTypes::ENUM => {String::from("enum")}
        }
    }
}

pub struct Token(pub TokenTypes, pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Token(TokenTypes::{:?}, '{}')", self.0, self.1)
    }
}