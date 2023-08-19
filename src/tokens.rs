#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // keywords
    DEP,    // project Dependency
    USE,    // import package
    TAG,    // tagging code blocks
    STRUCT, // struct with fields
    ENUM,   // list of identifiers
    VAR,    // mutable variable
    CONST,  // immutable variable
    FUNC,   // functions
    RETURN, // return statement
    FOR,    // for loop
    WHILE,  // while loop
    IF,     // if statement
    WHEN,   // when statement
    ELSE,   // else statement (if, when)
    TRY,    // try to run a block of code and catch exception if fails
    CATCH,  // catching when try fails

    // literals
    LOCAL,
    FALSE,
    TRUE,
    NONE,

    // other keywords
    IN,  // in
    AS,  // as
    AND, // and
    OR,  // or

    // identifier, literals
    IDENT,  // identifiers (names)
    NUMBER, // numbers
    STRING, // text

    // symbols
    // arithmetic operators and comparing
    PLUS,        // +
    MINUS,       // -
    DIVIDE,      // /
    MULTIPLY,    // *
    ASSIGN,      // =
    VARASSIGN,   // :=
    CONSTASSIGN, // ::

    EQUAL,              // ==
    NOTEQUAL,           // !=
    GREATERTHAN,        // >
    LESSTHAN,           // <
    GREATEROREQUALTHAN, // >=
    LESSOREQUALTHAN,    // <=

    // other symbols
    LCURLY,      // {
    RCURLY,      // }
    LPARENT,     // (
    RPARENT,     // )
    LSQUAREBRAC, // [
    RSQUAREBRAC, // ]
    COMMENT,     // //
    QUOTMARK,    // "
    APOSTROPHE,  // '
    BANG,        // !
    COMMA,       // ,
    DOT,         // .
    COLON,       // :
    ARROW,       // ->

    // other
    ILLEGAL, // illeagl expression
    EOL,     // end of line
    EOF,     // end of file
}

impl TokenType {
    pub fn literal(&self) -> String {
        // When changing token literals for symbols there might be some issue due to symbols with two characters like arrow or not-equal
        match self {
            TokenType::DEP => String::from("dep"),
            TokenType::USE => String::from("use"),
            TokenType::TAG => String::from("tag"),
            TokenType::STRUCT => String::from("struct"),
            TokenType::ENUM => String::from("enum"),
            TokenType::VAR => String::from("var"),
            TokenType::CONST => String::from("const"),
            TokenType::FUNC => String::from("func"),
            TokenType::RETURN => String::from("return"),
            TokenType::FOR => String::from("for"),
            TokenType::WHILE => String::from("while"),
            TokenType::IF => String::from("if"),
            TokenType::WHEN => String::from("when"),
            TokenType::ELSE => String::from("else"),
            TokenType::TRY => String::from("try"),
            TokenType::CATCH => String::from("catch"),

            TokenType::LOCAL => String::from("local"),
            TokenType::TRUE => String::from("true"),
            TokenType::FALSE => String::from("false"),
            TokenType::NONE => String::from("none"),

            TokenType::IN => String::from("in"),
            TokenType::AS => String::from("as"),
            TokenType::AND => String::from("and"),
            TokenType::OR => String::from("or"),

            TokenType::IDENT => String::from("IDENT"),
            TokenType::NUMBER => String::from("NUMBER"),
            TokenType::STRING => String::from("STRING"),

            TokenType::PLUS => String::from("+"),
            TokenType::MINUS => String::from("-"),
            TokenType::DIVIDE => String::from("/"),
            TokenType::MULTIPLY => String::from("*"),
            TokenType::ASSIGN => String::from("="),
            TokenType::VARASSIGN => String::from(":="),
            TokenType::CONSTASSIGN => String::from("::"),

            TokenType::EQUAL => String::from("=="),
            TokenType::NOTEQUAL => String::from("!="),
            TokenType::GREATERTHAN => String::from(">"),
            TokenType::LESSTHAN => String::from("<"),
            TokenType::GREATEROREQUALTHAN => String::from(">="),
            TokenType::LESSOREQUALTHAN => String::from("<="),

            TokenType::LCURLY => String::from("{"),
            TokenType::RCURLY => String::from("}"),
            TokenType::LPARENT => String::from("("),
            TokenType::RPARENT => String::from(")"),
            TokenType::LSQUAREBRAC => String::from("["),
            TokenType::RSQUAREBRAC => String::from("]"),
            TokenType::COMMENT => String::from("//"),
            TokenType::QUOTMARK => String::from("\""),
            TokenType::APOSTROPHE => String::from("'"),
            TokenType::BANG => String::from("!"),
            TokenType::COMMA => String::from(","),
            TokenType::DOT => String::from("."),
            TokenType::COLON => String::from(":"),
            TokenType::ARROW => String::from("->"),

            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::EOL => String::from("EOL"),
            TokenType::EOF => String::from("EOF"),
            // Not always unreachable so it is disabled for now
            #[allow(unreachable_patterns)]
            _ => {
                panic!("{:?} is missing a literal type", self)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub cur_pos: i32,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, cur_pos: usize) -> Self {
        Token {
            token_type: token_type,
            literal: literal,
            cur_pos: cur_pos as i32,
        }
    }
}
