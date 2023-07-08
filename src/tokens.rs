pub enum Tokens {
    // Keywords
    VAR,
    FUN,
    FOR,
    WHILE,
    IF,
    ELSE,

    // Special character
    LBRAC,       // {
    RBRAC,       // }
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
    INTEGER,
    STRING,
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
    GREATERTHAN, // >
    LESSERTHAN,    // <
    GREATEROREQUALTHAN, // >=
    LESSEROREQUALTHAN,    // <=

    // Structures
    STRUCT,
    ENUM,
}

impl Tokens {
    pub fn literal(&self) -> &str {
        match self {
            Tokens::VAR => {"var"}
            Tokens::FUN => {"fun"}
            Tokens::FOR => {"for"}
            Tokens::WHILE => {"while"}
            Tokens::IF => {"if"}
            Tokens::ELSE => {"else"}

            Tokens::IDENT => {"IDENT"}
            Tokens::INTEGER => {"INT"}
            Tokens::STRING => {"STRING"}
            Tokens::TRUE => {"true"}
            Tokens::FALSE => {"false"}

            Tokens::EOL => {"EOL"}
            Tokens::ILLEGAL => {"ILLEGAL"}

            Tokens::ASSIGN => {"="}
            Tokens::PLUS => {"+"}
            Tokens::MINUS => {"-"}
            Tokens::MULTIPLY => {"*"}
            Tokens::DIVIDE => {"/"}
            
            Tokens::EQUAL => {"=="}
            Tokens::GREATERTHAN => {">"}
            Tokens::LESSERTHAN => {"<"}
            Tokens::GREATEROREQUALTHAN => {">="}
            Tokens::LESSEROREQUALTHAN => {"<="}

            Tokens::COMMENT => {"//"}
            
            Tokens::COLON => {":"}
            Tokens::COMMA => {","}
            Tokens::LBRAC => {"{"}
            Tokens::RBRAC => {"}"}
            Tokens::LPARENT => {"("}
            Tokens::RPARENT => {")"}
            Tokens::LSQUAREBRAC => {"["}
            Tokens::RSQUAREBRAC => {"]"}
            Tokens::QUOTMARKS => {"\""}

            Tokens::STRUCT => {"struct"}
            Tokens::ENUM => {"enum"}
        }
    }
}
