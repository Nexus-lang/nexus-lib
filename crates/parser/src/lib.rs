pub mod ast;
mod tests;

use std::{error::Error, fmt::Display, mem::swap, process::ExitStatus, thread::panicking};

use ast::{ConstStmt, Expression, Ident, OptionallyTypedIdent, Statement, VarStmt};
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
                return self.parse_stmt();
            }
            Token::Eof => return Err(EofError),
            _ => {
                if let Token::Ident(_) = self.cur_tok {
                    dbg!("Peek: {}", &self.peek_tok);
                    if self.peek_tok == Token::VarAssign {
                        return Ok(self.parse_quick_assign(false));
                    } else if self.peek_tok == Token::ConstAssign {
                        return Ok(self.parse_quick_assign(true));
                    } else if self.peek_tok == Token::Colon {
                        todo!("Type annotations")
                    }
                }
                Statement::Expression(self.parse_expr(Precedence::LOWEST))
            }
        })
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Expression {
        match self.cur_tok {
            Token::Enum => todo!(),
            Token::Struct => todo!(),
            Token::Func => todo!(),
            Token::Loop => todo!(),
            Token::If => todo!(),
            Token::Else => todo!(),
            Token::When => todo!(),
            Token::ExclamMark => todo!(),
            Token::String(_, _) => todo!(),
            Token::Num(num) => Expression::Literal(ast::Literal::Num(num)),
            Token::Bool(_) => todo!(),
            Token::Ident(_) => todo!(),
            Token::Plus => todo!(),
            Token::Minus => todo!(),
            Token::LParent => todo!(),
            Token::LSquare => todo!(),
            Token::LCurly => todo!(),
            _ => panic!(
                "Received: {:#?}, not a valid token for an expression",
                self.cur_tok
            ),
        }
    }

    fn parse_variable(&mut self, is_const: bool) -> Statement {
        let name = Ident(match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!("Expected an identifier, received: {}", self.peek_tok),
        });

        self.next_token();

        let _type = if self.peek_tok == Token::Colon {
            self.next_token();
            let ident = Ident(self.peek_tok.to_string());
            self.next_token();
            self.expect_peek(Token::Assign);
            self.next_token();
            Some(ident)
        } else if self.peek_tok == Token::Assign {
            self.next_token();
            None
        } else {
            panic!("Expected Assing, received: {}", self.peek_tok)
        };

        self.next_token();

        let val = self.parse_expr(Precedence::LOWEST);

        match is_const {
            true => Statement::Var(VarStmt {
                name: OptionallyTypedIdent {
                    ident: name,
                    _type,
                },
                val,
            }),
            false => Statement::Const(ConstStmt {
                name: OptionallyTypedIdent {
                    ident: name,
                    _type,
                },
                val,
            }),
        }
    }

    fn parse_quick_assign(&mut self) -> Statement {
        let name = Ident(match self.cur_tok {
            Token::Ident(_) => self.cur_tok.to_string(),
            // unreachable
            _ => panic!("Expected an identifier, received: {}", self.cur_tok),
        });

        let _type = if self.peek_tok == Token::Colon {
            self.next_token();
            let ident = Ident(self.peek_tok.to_string());
            self.next_token();
            self.expect_peek(Token::Assign);
            self.next_token();
            Some(ident)
        } else {
            panic!("Expected Assing, received: {}", self.peek_tok)
        };

        self.next_token();

        let val = self.parse_expr(Precedence::LOWEST);

        match is_const {
            true => Statement::Var(VarStmt {
                name: OptionallyTypedIdent {
                    ident: name,
                    _type,
                },
                val,
            }),
            false => Statement::Const(ConstStmt {
                name: OptionallyTypedIdent {
                    ident: name,
                    _type,
                },
                val,
            }),
        }
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
