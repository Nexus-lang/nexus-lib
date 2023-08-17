use std::{process, collections::HashMap};

use crate::{
    ast::{Identifier, Program, Statement, VarStatement, ReturnStatement, ExpressionStatement, Expression},
    lexer::Lexer,
    tokens::{Token, TokenType},
};

pub struct Parser {
    token_stream: Vec<Token>,
    lexer: Lexer,

    cur_token: Token,
    peek_token: Token,
    current_pos: usize,
    errors: Vec<String>,
    line_count: i32,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        let token_stream: Vec<Token> = lexer.lex();
        let lexer = lexer.clone();
        return Parser {
            cur_token: token_stream[0].clone(),
            peek_token: token_stream[1].clone(),
            current_pos: 0,
            errors: vec![],
            token_stream: token_stream,
            lexer,
            line_count: 1,
        };
    }

    fn next_token(&mut self) {
        self.current_pos += 1;
        self.cur_token = self.peek_token.clone();
        if self.cur_token_is(TokenType::EOL) {
            self.line_count += 1;
        }
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
        println!("{:?}", self.errors());

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::VAR => self.parse_var_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            // might have to be improved in the future
            TokenType::ILLEGAL => {
                let msg = format!("Illegal token: '{}' at: {}:{}:{} is not a valid token", self.cur_token.literal, self.lexer.input.file_path, self.line_count, self.token_stream[self.current_pos + 1].cur_pos + 1,);
                self.throw_error(msg);
                None
            }
            _ => {
                self.parse_expression_statement()
            },
        }
    }

    fn parse_var_statement(&mut self) -> Option<Statement> {
        let mut statement = VarStatement {
            name: Identifier {
                value: "".to_string(),
            },
            value: None,
        };
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        statement.name = Identifier {
            value: self.cur_token.clone().literal,
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token();

        Some(Statement::VAR(statement))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement = ReturnStatement { return_value: None };

        self.next_token();

        // TODO: Expression parsing

        Some(Statement::RETURN(statement))
    }

    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let statement = ExpressionStatement{expression: self.parse_expression(LOWEST)};
        self.next_token();
        statement
    }

    fn parse_expression(&self) -> Expression {
        
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, found {:?} instead error here: {}:{}:{}",
            token_type, self.peek_token.token_type, self.lexer.input.file_path, self.line_count, self.token_stream[self.current_pos + 1].cur_pos + 1,
        );
        self.errors.push(msg);
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn throw_error(&mut self, message: String) {
        self.errors.push(message);
        println!("{:?}", self.errors());
        process::exit(1);
    } 
}
