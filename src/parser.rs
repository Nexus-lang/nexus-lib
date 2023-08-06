use crate::{
    ast::{Program, Statement, VarStatement, Identifier, Expression, self},
    lexer::Lexer,
    tokens::{Token, TokenType},
};

pub struct Parser {
    token_stream: Vec<Token>,

    cur_token: Token,
    peek_token: Token,
    current_pos: usize,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        let token_stream: Vec<Token> = lexer.lex();
        return Parser {
            cur_token: token_stream[0].clone(),
            peek_token: token_stream[0].clone(),
            current_pos: 0,
            token_stream: token_stream,
        };
    }

    fn next_token(&mut self) {
        self.current_pos += 1;
        self.cur_token = self.peek_token.clone();
        if self.current_pos + 1 < self.token_stream.len() {
            self.peek_token = self.token_stream[self.current_pos + 1].clone();
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new(vec![]);

        while self.cur_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();
            if statement != None {
                program.statements.push(statement.unwrap());
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::VAR => {
                self.parse_var_statement()
            }
            _ => {
                None
            }
        }
    }

    fn parse_var_statement(&self) -> Option<Statement> {
        let mut statement = VarStatement { token: self.cur_token.clone(), name: Identifier { token: TokenType::ILLEGAL, value: "".to_string() }, value: Expression::EMPTY };
        if self.expect_peek(TokenType::IDENT) {
            return None;
        }

        statement.name = Identifier { token: self.cur_token, value: self.cur_token.literal };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::EOL) {
            self.next_token();
        }

        Some(Statement::VAR(statement))
    }

    fn expect_peek() {

    }

    fn cur_token_is() {
        
    }
}
