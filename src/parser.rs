use crate::tokens::Token;

struct Parser {
    token_stream: Vec<Token>,
    cur_token: Token,
    peek_token: Token
}

