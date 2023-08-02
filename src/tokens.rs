#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Keywords
    VAR,
    CONST,
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
    ENUM,
}

impl TokenType {
    pub fn literal(&self) -> String {
        match self {
            TokenType::VAR => {String::from("var")}
            TokenType::CONST => {String::from("const")}
            TokenType::FUNC => {String::from("func")}
            TokenType::FOR => {String::from("for")}
            TokenType::WHILE => {String::from("while")}
            TokenType::IF => {String::from("if")}
            TokenType::ELSE => {String::from("else")}
            TokenType::WHEN => {String::from("when")}
            TokenType::USE => {String::from("use")}

            TokenType::IDENT => {String::from("IDENT")}
            TokenType::NUMBER => {String::from("NUMBER")}
            TokenType::TRUE => {String::from("true")}
            TokenType::FALSE => {String::from("false")}

            TokenType::EOL => {String::from("EOL")}
            TokenType::ILLEGAL => {String::from("ILLEGAL")}

            TokenType::ASSIGN => {String::from("=")}
            TokenType::PLUS => {String::from("+")}
            TokenType::MINUS => {String::from("-")}
            TokenType::MULTIPLY => {String::from("*")}
            TokenType::DIVIDE => {String::from("/")}
            
            TokenType::EQUAL => {String::from("==")}
            TokenType::NOTEQUAL => {String::from("!=")}
            TokenType::GREATERTHAN => {String::from(">")}
            TokenType::LESSERTHAN => {String::from("<")}
            TokenType::GREATEROREQUALTHAN => {String::from(">=")}
            TokenType::LESSEROREQUALTHAN => {String::from("<=")}

            TokenType::AND => {String::from("and")}
            TokenType::OR => {String::from("or")}
            TokenType::IN => {String::from("in")}
            TokenType::ARROW => {String::from("->")}
            TokenType::OTHER => {String::from("other")}

            TokenType::COMMENT => {String::from("//")}
            
            TokenType::COLON => {String::from(":")}
            TokenType::COMMA => {String::from(",")}
            TokenType::LCURLY => {String::from("{")}
            TokenType::RCURLY => {String::from("}")}
            TokenType::LPARENT => {String::from("(")}
            TokenType::RPARENT => {String::from(")")}
            TokenType::LSQUAREBRAC => {String::from("[")}
            TokenType::RSQUAREBRAC => {String::from("]")}
            TokenType::QUOTMARK => {String::from("\"")}
            TokenType::EXCLAMMARK => {String::from("!")}

            TokenType::STRUCT => {String::from("struct")}
            TokenType::ENUM => {String::from("enum")}
            // Not always unreachable so it is disabled for now
            #[allow(unreachable_patterns)]
            _ => {
                panic!("{:?} is missing a literal type", self)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token(pub TokenType, pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Token(TokenTypes::{:?}, '{}')", self.0, self.1)
    }
}
