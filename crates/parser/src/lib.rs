pub mod ast;
mod tests;

use std::{mem::swap, thread::panicking, error::Error, fmt::Display};

use ast::{Statement, ConstStmt};
use lexer::{tokens::Token, Lexer};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

enum Precedence {
    /// default
    LOWEST,
    /// Assign new value to variable
    ASSIGN,
    /// Check if i is in list/range
    ///
    /// `i in 0..10`
    CONTAINS,
    /// Range of numbers
    /// 0..1000
    RANGE,
    /// Check if value is equivalent
    /// to other value
    ///
    /// `x == 0`
    EQUALS,
    /// Lesser and greater
    /// comparison operations
    LESSGREATER,
    /// Lesser or equal and greater or equal
    /// comparsion operations
    LESSGREATEROREQUAL,
    /// Sum of two numbers
    SUM,
    /// Product of two numbers
    PRODUCT,
    /// Prefix operators like +, -, !
    PREFIX,
    /// Call a function
    CALL,
    /// Convert types using `as`
    CONVERSION,
    /// Index a list
    /// `myList[1]`
    INDEX,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = lexer.tokenize();
        let peek_tok = lexer.tokenize();
        Self {
            lexer,
            cur_tok,
            peek_tok,
        }
    }

    pub fn parse_stmt(&mut self) -> Result<Statement, EofError> {
        Ok(match self.cur_tok {
            Token::Use => todo!(),
            Token::Var => self.parse_variable(false),
            Token::Const => self.parse_variable(true),
            Token::Break => todo!(),
            Token::Return => todo!(),
            Token::Local => todo!(),
            Token::Eol => {
                self.next_token();
                return self.parse_stmt()
            },
            Token::Eof => return Err(EofError),
            _ => todo!(),
        })
    }

    fn parse_variable(&mut self, is_const: bool) -> Statement {
        let name = match self.peek_tok {
            Token::Ident(_) => self.cur_tok.to_string(),
            _ => panic!("Expected an identifier, received: {}", self.peek_tok)
        };
        
        self.next_token();

        self.expect_peek(Token::Assign);

        self.next_token();

        self.next_token();

        let val = self.parse_expr();

        todo!()
    }

    fn expect_peek(&self, expected: Token) {
        if self.peek_tok != expected {
            panic!("Expected: {}, received: {}", expected, self.peek_tok)
        }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = self.lexer.tokenize();
    }
}

#[derive(Debug)]
pub struct EofError;

impl Error for EofError {}

impl Display for EofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Encountered end of file")
    }
}