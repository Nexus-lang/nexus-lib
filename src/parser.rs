use crate::{lexer::Lexer, tokens::Token};

pub struct Parser {
    token_stream: Vec<Token>,
    lexer: Lexer,

    cur_token: Token,
    peek_token: Token,
    current_pos: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token_stream: Vec<Token> = lexer.lex();
        println!("{:?}", token_stream[1]);
        return Parser { lexer: lexer, cur_token: token_stream[0].clone(), peek_token: token_stream[1].clone(), current_pos: 0, token_stream: token_stream};
    }

    pub fn next_token(&mut self) {
        self.current_pos += 1;
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.token_stream[self.current_pos + 1].clone();
        println!("{:?}", self.peek_token);
    }
}
