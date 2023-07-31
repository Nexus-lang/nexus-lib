use crate::{lexer::lex, tokens::TokenType};

struct Parser {
    cur_token: TokenType,
    peek_token: TokenType
}

