pub mod ast;
mod tests;

use std::mem::swap;

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

    pub fn parse_stmt() {}

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = self.lexer.tokenize();
    }
}
