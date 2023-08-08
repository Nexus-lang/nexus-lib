#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    APOSTROPHE, // '
    EXCLAMMARK,
    COMMA,       // ,
    DOT,         // .
    COLON,       // :

    // Identifier, literals
    IDENT,
    NUMBER,
    STRING,
    FALSE,
    TRUE,

    // Other
    ILLEGAL, // illeagl expression
    EOL,     // End of line
    EOF,     // End of file

    // Arithmetic operators and alike
    PLUS,        // +
    MINUS,       // -
    DIVIDE,      // /
    MULTIPLY,    // *
    ASSIGN,      // =
    CONSTASSIGN, // c:

    EQUAL,       // ==
    NOTEQUAL,    // !=
    GREATERTHAN, // >
    LESSTHAN,    // <
    GREATEROREQUALTHAN, // >=
    LESSOREQUALTHAN,    // <=

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
        // When changing token literals for symbols there might be some issue due to symbols with two characters like arrow or not-equal
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
            TokenType::STRING => {String::from("STRING")}
            TokenType::TRUE => {String::from("true")}
            TokenType::FALSE => {String::from("false")}

            TokenType::EOL => {String::from("EOL")}
            TokenType::EOF => {String::from("EOF")}
            TokenType::ILLEGAL => {String::from("ILLEGAL")}

            TokenType::ASSIGN => {String::from("=")}
            TokenType::CONSTASSIGN => {String::from("c:")}
            TokenType::PLUS => {String::from("+")}
            TokenType::MINUS => {String::from("-")}
            TokenType::MULTIPLY => {String::from("*")}
            TokenType::DIVIDE => {String::from("/")}

            TokenType::EQUAL => {String::from("==")}
            TokenType::NOTEQUAL => {String::from("!=")}
            TokenType::GREATERTHAN => {String::from(">")}
            TokenType::LESSTHAN => {String::from("<")}
            TokenType::GREATEROREQUALTHAN => {String::from(">=")}
            TokenType::LESSOREQUALTHAN => {String::from("<=")}

            TokenType::AND => {String::from("and")}
            TokenType::OR => {String::from("or")}
            TokenType::IN => {String::from("in")}
            TokenType::ARROW => {String::from("->")}
            TokenType::OTHER => {String::from("other")}

            TokenType::COMMENT => {String::from("//")}

            TokenType::COLON => {String::from(":")}
            TokenType::COMMA => {String::from(",")}
            TokenType::DOT => {String::from(".")}
            TokenType::LCURLY => {String::from("{")}
            TokenType::RCURLY => {String::from("}")}
            TokenType::LPARENT => {String::from("(")}
            TokenType::RPARENT => {String::from(")")}
            TokenType::LSQUAREBRAC => {String::from("[")}
            TokenType::RSQUAREBRAC => {String::from("]")}
            TokenType::QUOTMARK => {String::from("\"")}
            TokenType::APOSTROPHE => {String::from("'")}
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub cur_pos: i32,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, cur_pos: usize) -> Self {
        Token { token_type: token_type, literal: literal, cur_pos: cur_pos as i32 }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
