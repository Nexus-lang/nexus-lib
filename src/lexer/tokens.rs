use crate::util::FirstAsChar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // keywords
    DEP,    // project Dependency
    USE,    // import package
    STRUCT, // struct with fields
    ENUM,   // list of identifiers
    VAR,    // mutable variable
    CONST,  // immutable variable
    FUNC,   // functions
    RETURN, // return statement
    LOOP,    // loop
    IF,     // if statement
    WHEN,   // when statement
    ELSE,   // else statement (if, when)
    TRY,    // try to run a block of code and catch exception if fails
    CATCH,  // catching when try fails

    // literals
    LOCAL, // declare var, func... as private
    FALSE, // boolean literal false
    TRUE,  // boolean literal true
    NONE,  // empty value none

    // other keywords
    IN,  // in
    AS,  // as
    AND, // and
    OR,  // or

    // identifier, literals
    IDENT, // identifiers (names)
    // reference to a variable, function, class, enum...
    STRINGREFB, // beginning
    STRINGREFE, // end

    NUMBER, // numbers
    STRING, // text

    STRINGB,
    STRINGE,

    // symbols
    // arithmetic operators and comparing
    PLUS,        // +
    MINUS,       // -
    DIVIDE,      // /
    MULTIPLY,    // *
    ASSIGN,      // =
    VARASSIGN,   // :=
    CONSTASSIGN, // ::
    RANGE,       // x..x

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
    SEMICOLON,   // ;
    ANNOTATION,  // @

    // other
    ILLEGAL, // illeagl expression
    EOL,     // end of line
    EOF,     // end of file
}

impl TokenType {
    /// return literal values of tokens.
    // these will be used to construct Token list in lexer
    pub fn literal(&self) -> String {
        // When changing token literals for symbols there might be some issue due to symbols with two characters like arrow or not-equal
        match self {
            TokenType::DEP => String::from("dep"),
            TokenType::USE => String::from("use"),
            TokenType::STRUCT => String::from("struct"),
            TokenType::ENUM => String::from("enum"),
            TokenType::VAR => String::from("var"),
            TokenType::CONST => String::from("const"),
            TokenType::FUNC => String::from("func"),
            TokenType::RETURN => String::from("return"),
            TokenType::LOOP => String::from("loop"),
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
            TokenType::STRINGREFB => String::from("STRINGREFB"), // beginning
            TokenType::STRINGREFE => String::from("STRINGREFE"), // end
            TokenType::NUMBER => String::from("NUMBER"),
            TokenType::STRING => String::from("STRING"),
            TokenType::STRINGB => String::from("STRINGB"),
            TokenType::STRINGE => String::from("STRINGE"),

            TokenType::PLUS => String::from("+"),
            TokenType::MINUS => String::from("-"),
            TokenType::DIVIDE => String::from("/"),
            TokenType::MULTIPLY => String::from("*"),
            TokenType::ASSIGN => String::from("="),
            TokenType::VARASSIGN => String::from(":="),
            TokenType::CONSTASSIGN => String::from("::"),
            TokenType::RANGE => String::from(".."),

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
            TokenType::SEMICOLON => String::from(";"),
            TokenType::ANNOTATION => String::from("@"),

            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::EOL => String::from("EOL"),
            TokenType::EOF => String::from("EOF"),
        }
    }
}

impl FirstAsChar for TokenType {
    fn first_as_char(&self) -> char {
        let chars: Vec<char> = self.literal().chars().collect();
        if chars.len() > 0 {
            chars[0]
        } else {
            panic!("Cannot convert empty string to char")
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
    /// constructs new token
    // this is then appended to the token list
    pub fn new(token_type: TokenType, literal: String, cur_pos: usize) -> Self {
        Token {
            literal: literal,
            token_type,
            cur_pos: cur_pos as i32,
        }
    }
}
