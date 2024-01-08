pub mod ast;
mod tests;

use std::{collections::VecDeque, error::Error, fmt::Display, mem::swap, io};

use ast::{ConstStmt, Expression, Ident, OptionallyTypedIdent, Statement, VarStmt};
use lexer::{tokens::{Token, Literal}, Lexer};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let iself = *self as i8;
        let iother = *other as i8;
        iself.partial_cmp(&iother)
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let iself = *self as i8;
        let iother = *other as i8;
        iself.cmp(&iother)
    }
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
                        return Ok(self.parse_quick_assign());
                    } else if self.peek_tok == Token::ConstAssign {
                        return Ok(self.parse_quick_assign());
                    } else if self.peek_tok == Token::Colon {
                        todo!("Type annotations")
                    }
                }
                Statement::Expression(self.parse_expr(Precedence::LOWEST))
            }
        })
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Expression {
        let prefix = self.parse_prefix();
        if let None = prefix {
            panic!("No prefix parse found for: {}", self.cur_tok)
        }

        let prefix = prefix.unwrap();
        let mut left_expression = prefix;

        while !self.peek_is_end() && precedence < self.get_precedence(&self.peek_tok) {
            self.next_token();
            // Unwrap here might not be safe. Observe this
            left_expression = match self.parse_infix(left_expression) {
                Some(expr) => expr,
                None => panic!("Expr is empty"),
            };
        }

        left_expression
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_infix(&mut self, left_expr: Expression) -> Option<Expression> {
        todo!()
    }

    fn parse_string(&mut self) -> Expression {
        // TODO: Parse references
        Expression::Literal(Literal::Str(self.cur_tok.to_string()))
    }

    fn parse_variable(&mut self, is_const: bool) -> Statement {
        let name = Ident(match self.peek_tok {
            Token::Ident(_) => self.peek_tok.to_string(),
            _ => panic!("Expected an identifier, received: {}", self.peek_tok),
        });

        self.next_token();

        let _type = match self.peek_tok {
            Token::Colon => {
                self.next_token();
                let ident = Ident(self.peek_tok.to_string());
                self.next_token();
                self.expect_peek(Token::Assign);
                self.next_token();
                Some(ident)
            }
            Token::Assign => {
                self.next_token();
                None
            }
            _ => panic!("Expected Assign, received: {}", self.peek_tok),
        };

        self.next_token();

        let val = self.parse_expr(Precedence::LOWEST);

        match is_const {
            true => Statement::Const(ConstStmt {
                name: OptionallyTypedIdent { ident: name, _type },
                val,
            }),
            false => Statement::Var(VarStmt {
                name: OptionallyTypedIdent { ident: name, _type },
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

        let is_const;

        let _type = match self.peek_tok {
            Token::Colon => {
                self.next_token();
                let ident = Ident(self.peek_tok.to_string());
                self.next_token();
                match self.peek_tok {
                    Token::ConstAssign => is_const = true,
                    Token::VarAssign => is_const = false,
                    _ => panic!(
                        "Expected ConstAssign or VarAssign, received: {}",
                        self.peek_tok
                    ),
                }
                Some(ident)
            }
            Token::ConstAssign => {
                is_const = true;
                None
            }
            Token::VarAssign => {
                is_const = false;
                None
            }
            _ => panic!("Expected Assign, received: {}", self.peek_tok),
        };

        self.next_token();
        self.next_token();

        let val = self.parse_expr(Precedence::LOWEST);

        match is_const {
            true => Statement::Const(ConstStmt {
                name: OptionallyTypedIdent { ident: name, _type },
                val,
            }),
            false => Statement::Var(VarStmt {
                name: OptionallyTypedIdent { ident: name, _type },
                val,
            }),
        }
    }

    fn expect_peek(&self, expected: Token) {
        if self.peek_tok != expected {
            panic!("Expected: {}, received: {}", expected, self.peek_tok)
        }
    }

    fn peek_is_end(&self) -> bool {
        match self.peek_tok {
            Token::Eol | Token::Eof => true,
            _ => false,
        }
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = self.lexer.tokenize();
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Assign => Precedence::ASSIGN,
            Token::Equals | Token::NotEquals => Precedence::EQUALS,
            Token::Greater | Token::Lesser => Precedence::LESSGREATER,
            Token::GreaterEquals | Token::LesserEquals => Precedence::LESSGREATEROREQUAL,
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Asterisk | Token::Slash => Precedence::PRODUCT,
            Token::LParent => Precedence::CALL,
            Token::LSquare => Precedence::INDEX,
            _ => Precedence::LOWEST,
        }
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
