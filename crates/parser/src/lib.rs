pub mod ast;
mod tests;

use std::{error::Error, fmt::Display, mem::swap};

use ast::{BlockStmt, ConstStmt, Expression, Ident, OptionallyTypedIdent, Statement, VarStmt};
use lexer::{
    tokens::{Literal, Operator, Token},
    Lexer,
};

use crate::ast::FuncExpr;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Precedence {
    /// default value
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
        let iself = *self as u8;
        let iother = *other as u8;
        iself.partial_cmp(&iother)
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let iself = *self as u8;
        let iother = *other as u8;
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
                    if self.peek_tok == Token::VarAssign {
                        return Ok(self.parse_quick_assign());
                    }
                    match self.peek_tok {
                        Token::Colon | Token::ConstAssign | Token::VarAssign => {
                            return Ok(self.parse_quick_assign())
                        }
                        _ => (),
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
        Some(match self.cur_tok {
            Token::Ident(_) => Expression::Ident(Ident(self.cur_tok.to_string())),
            Token::Literal(Literal::Bool(ref bool)) => Expression::Literal(Literal::Bool(*bool)),
            Token::Literal(Literal::Num(ref lit)) => Expression::Literal(Literal::Num(*lit)),
            Token::Literal(Literal::Str(_)) => self.parse_str_lit(),
            Token::LSquare => self.parse_list_lit(),
            // TODO: add hashes
            /*
            TokenType::LCURLY => self.parse_hash_literal(),
            */
            // Token::NONE => Expression::NONE(NoneLiteral),
            Token::LParent => self.parse_grouped_expr(),
            Token::Func => self.parse_func_expr(),
            Token::If => self.parse_if_expr(),
            Token::Loop => self.parse_loop_expr(),
            Token::When => self.parse_when_expr(),
            Token::ExclamMark
            | Token::Operator(Operator::Plus)
            | Token::Operator(Operator::Minus) => self.parse_prefix_expr(),
            // Token::ANNOTATION => self.parse_annotation(),
            _ => return None,
        })
    }

    fn parse_infix(&mut self, left_expr: Expression) -> Option<Expression> {
        todo!()
    }

    fn parse_str_lit(&mut self) -> Expression {
        // TODO: Parse references
        Expression::Literal(Literal::Str(self.cur_tok.to_string()))
    }

    fn parse_list_lit(&mut self) -> Expression {
        todo!()
    }

    fn parse_grouped_expr(&mut self) -> Expression {
        todo!()
    }

    fn parse_func_expr(&mut self) -> Expression {
        self.expect_peek(Token::LParent);
        self.next_token();
        let args = self.parse_ident_list(Token::RParent);
        let ret_type = match self.peek_tok {
            Token::Colon => {
                self.next_token();
                self.next_token();
                Some(Ident(self.cur_tok.to_string()))
            },
            Token::LCurly => None,
            _ => panic!()
        };
        self.next_token();
        let block = self.parse_block_stmt();
        Expression::Func(FuncExpr {
            ret_type,
            args,
            block,
        })
    }

    fn parse_if_expr(&mut self) -> Expression {
        todo!()
    }

    fn parse_loop_expr(&mut self) -> Expression {
        todo!()
    }

    fn parse_when_expr(&mut self) -> Expression {
        todo!()
    }

    fn parse_prefix_expr(&mut self) -> Expression {
        todo!()
    }

    /// First token needs to be the begin_token like `(` or `{` for example
    /// This function sets cur_tok to the end_tok
    fn parse_ident_list(&mut self, end_tok: Token) -> Vec<OptionallyTypedIdent> {
        self.next_token();
        let mut items = Vec::new();
        let first_item = self.parse_typed_ident();
        items.push(first_item);
        if self.peek_tok == Token::Comma {
            self.next_token();
        }
        while self.peek_tok != end_tok {
            self.next_token();
            let ident = self.parse_typed_ident();
            items.push(ident);
            if self.peek_tok == Token::Comma {
                self.next_token();
            }
        }
        self.next_token();
        items
    }

    /// First token needs to be a left curly `{`
    fn parse_block_stmt(&mut self) -> BlockStmt {
        let mut stmts = Vec::new();
        while self.peek_tok != Token::RCurly {
            self.next_token();
            let stmt = self
                .parse_stmt()
                .expect("Found eof even though the blockstatement was not yet fully parsed");
            stmts.push(stmt);
        }
        BlockStmt { stmts }
    }

    fn parse_typed_ident(&mut self) -> OptionallyTypedIdent {
        let ident = Ident(self.cur_tok.to_string());
        let _type = match self.peek_tok {
            Token::Colon => {
                self.next_token();
                self.next_token();
                Some(Ident(self.cur_tok.to_string()))
            }
            _ => None,
        };
        OptionallyTypedIdent { ident, _type }
    }

    /// First token needs to be the begin_token like `(` or `{` for example
    fn parse_raw_list(&mut self, end_tok: Token) -> Vec<Expression> {
        todo!()
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
            Token::Operator(op) => match op {
                Operator::Equals | Operator::NotEquals => Precedence::EQUALS,
                Operator::Greater | Operator::Lesser => Precedence::LESSGREATER,
                Operator::GreaterEquals | Operator::LesserEquals => Precedence::LESSGREATEROREQUAL,
                Operator::Plus | Operator::Minus => Precedence::SUM,
                Operator::Asterisk | Operator::Slash => Precedence::PRODUCT,
            },
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
