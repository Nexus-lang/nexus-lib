#[derive(Debug)]
pub enum TokenTypes {
    // Keywords
    VAR,
    FUNC,
    FOR,
    WHILE,
    IF,
    ELSE,
    WHEN,
    USE,


    // Other keywords
    

    // Special character
    LCURLY,       // {
    RCURLY,       // }
    LPARENT,     // (
    RPARENT,     // )
    LSQUAREBRAC, // [
    RSQUAREBRAC, // ]
    COMMENT,     // //
    QUOTMARK,   // "
    EXCLAMMARK,
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

    AND,
    OR,
    IN,
    ARROW, // ->
    OTHER,

    // Structures
    STRUCT,
    ENUM
}

impl TokenTypes {
    pub fn literal(&self) -> String {
        match self {
            TokenTypes::VAR => {String::from("var")}
            TokenTypes::FUNC => {String::from("func")}
            TokenTypes::FOR => {String::from("for")}
            TokenTypes::WHILE => {String::from("while")}
            TokenTypes::IF => {String::from("if")}
            TokenTypes::ELSE => {String::from("else")}
            TokenTypes::WHEN => {String::from("when")}
            TokenTypes::USE => {String::from("use")}

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

            TokenTypes::AND => {String::from("and")}
            TokenTypes::OR => {String::from("or")}
            TokenTypes::IN => {String::from("in")}
            TokenTypes::ARROW => {String::from("->")}
            TokenTypes::OTHER => {String::from("other")}

            TokenTypes::COMMENT => {String::from("//")}
            
            TokenTypes::COLON => {String::from(":")}
            TokenTypes::COMMA => {String::from(",")}
            TokenTypes::LCURLY => {String::from("{")}
            TokenTypes::RCURLY => {String::from("}")}
            TokenTypes::LPARENT => {String::from("(")}
            TokenTypes::RPARENT => {String::from(")")}
            TokenTypes::LSQUAREBRAC => {String::from("[")}
            TokenTypes::RSQUAREBRAC => {String::from("]")}
            TokenTypes::QUOTMARK => {String::from("\"")}
            TokenTypes::EXCLAMMARK => {String::from("!")}

            TokenTypes::STRUCT => {String::from("struct")}
            TokenTypes::ENUM => {String::from("enum")}
            // Not always unreachable so it is disabled for now
            #[allow(unreachable_patterns)]
            _ => {
                panic!("{:?} is missing a literal type", self)
            }
        }
    }
}

#[derive(Debug)]
pub struct Token(pub TokenTypes, pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Token(TokenTypes::{:?}, '{}')", self.0, self.1)
    }
}
