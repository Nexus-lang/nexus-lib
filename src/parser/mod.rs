pub mod ast;
mod tests;

use std::{error::Error, fmt::Display, mem::swap};

use crate::{lexer::{tokens::*, Lexer}, util};
use ast::*;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    cur_tok: Token,
    peek_tok: Token,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
    /// default value
    Lowest,
    /// Assign new value to variable
    Assign,
    /// Check if i is in list/range
    ///
    /// `i in 0..10`
    Contains,
    /// Range of numbers
    /// 0..1000
    Range,
    /// Check if value is equivalent
    /// to other value
    ///
    /// `x == 0`
    Equals,
    /// Lesser and greater
    /// comparison operations
    LessGreater,
    /// Lesser or equal and greater or equal
    /// comparsion operations
    LessGreaterOrEqual,
    /// Sum of two numbers
    Sum,
    /// Product of two numbers
    Product,
    /// Prefix operators like +, -, !
    Prefix,
    /// Call a function
    Call,
    /// Convert types using `as`
    Conversion,
    /// Index a list
    /// `myList[1]`
    Index,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = util::get_next_tok(lexer);
        let peek_tok = util::get_next_tok(lexer);
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
            Token::Break => {
                let label = match self.peek_tok {
                    Token::Ident(_) => {
                        self.next_token();
                        Some(Ident(self.cur_tok.to_string()))
                    }
                    _ => None,
                };
                Statement::Break(BreakStmt { label })
            }
            Token::Return => {
                let val = match self.peek_tok {
                    Token::Eol => None,
                    _ => {
                        self.next_token();
                        Some(self.parse_expr(Precedence::Lowest))
                    }
                };
                Statement::Return(ReturnStmt { val })
            }
            Token::Local => {
                if self.peek_tok == Token::Local {
                    panic!("Cannot stack multiple `local` statements")
                }
                self.next_token();
                let stmt = self.parse_stmt().expect(
                    "Encountered End of file instead of a statement after the local statement",
                );
                Statement::Local(LocalStmt {
                    val: Box::new(stmt),
                })
            }
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
                Statement::Expression(self.parse_expr(Precedence::Lowest))
            }
        })
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Expression {
        let prefix = self.parse_prefix();
        if prefix.is_none() {
            panic!("No prefix parse found for: {}", self.cur_tok)
        }

        let prefix = prefix.unwrap();
        let mut left_expression = prefix;

        while !self.peek_is_end() && precedence < self.get_precedence(&self.peek_tok) {
            self.next_token();
            // Unwrap here might not be safe. Observe this
            left_expression = match self.parse_infix(left_expression) {
                Some(expr) => expr,
                None => panic!("Invalid infix expression"),
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
            Token::LCURLY => self.parse_hash_literal(),
            */
            // Token::NONE => Expression::NONE(NoneLiteral),
            Token::LParent => self.parse_grouped_expr(),
            Token::Func => self.parse_func_expr(),
            Token::If => self.parse_if_expr(IfType::If),
            Token::Loop => self.parse_loop_expr(),
            Token::When => self.parse_when_expr(),
            Token::ExclamMark
            | Token::Operator(Operator::Plus)
            | Token::Operator(Operator::Minus) => self.parse_prefix_expr(),
            // Token::ANNOTATION => self.parse_annotation(),
            _ => return None,
        })
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        Some(match self.cur_tok {
            Token::Operator(ref op) => match op {
                Operator::Equals
                | Operator::NotEquals
                | Operator::Greater
                | Operator::Lesser
                | Operator::GreaterEquals
                | Operator::LesserEquals
                | Operator::Plus
                | Operator::Minus
                | Operator::Asterisk
                | Operator::Slash => self.parse_infix_expr(left),
            },
            Token::LParent => self.parse_call_expr(left),
            // Token::LSquare => self.parse_index_expr(left),
            _ => return None,
        })
    }

    fn parse_str_lit(&mut self) -> Expression {
        // TODO: Parse string interpolation
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
            }
            Token::LCurly => None,
            _ => panic!(),
        };
        self.next_token();
        let block = self.parse_block_stmt();
        Expression::Func(FuncExpr {
            ret_type,
            args,
            block,
        })
    }

    fn parse_if_expr(&mut self, _type: IfType) -> Expression {
        match _type {
            // Current token needs to be `if`
            IfType::If => {
                self.next_token();
                let cond = self.parse_expr(Precedence::Lowest);
                self.expect_peek(Token::LCurly);
                self.next_token();
                let block = self.parse_block_stmt();
                let alt = match self.peek_tok {
                    Token::Else => {
                        self.next_token();
                        Some(Box::from(match self.peek_tok {
                            Token::If => match self.parse_if_expr(IfType::ElseIf) {
                                Expression::If(_if) => _if,
                                _ => panic!("UNREACHABLE"),
                            },
                            Token::LCurly => match self.parse_if_expr(IfType::Else) {
                                Expression::If(_if) => _if,
                                _ => panic!("UNREACHABLE"),
                            },
                            ref other => {
                                panic!("Exptected `block` or `if` after else, got `{other:?}`")
                            }
                        }))
                    }
                    _ => None,
                };
                Expression::If(IfExpr {
                    cond: Some(Box::from(cond)),
                    block,
                    _type,
                    alt,
                })
            }
            // Current token needs to be `else`
            IfType::ElseIf => {
                self.next_token();
                let _if = match self.parse_if_expr(IfType::If) {
                    Expression::If(_if) => _if,
                    other => panic!("Unreachable: Got {other:?} instead of if expression"),
                };
                Expression::If(IfExpr { _type, .._if })
            }
            // Current token needs to be `else`
            IfType::Else => {
                self.next_token();
                let block = self.parse_block_stmt();
                Expression::If(IfExpr {
                    _type,
                    cond: None,
                    block,
                    alt: None,
                })
            }
        }
    }

    // TODO: parse else branches
    fn parse_loop_expr(&mut self) -> Expression {
        self.next_token();
        let cond = self.parse_expr(Precedence::Lowest);
        self.expect_peek(Token::LCurly);
        self.next_token();
        let block = self.parse_block_stmt();
        Expression::Loop(LoopExpr {
            _type: LoopType::While,
            cond: Some(Box::from(cond)),
            block,
            alt: None,
        })
    }

    fn parse_when_expr(&mut self) -> Expression {
        todo!()
    }

    fn parse_infix_expr(&mut self, left_expr: Expression) -> Expression {
        let op = match self.cur_tok {
            Token::Operator(_) => self.cur_tok_to_in_op(),
            ref other => panic!("Missing operator, got {other} instead"),
        };
        let prec = self.get_precedence(&self.cur_tok);
        self.next_token();
        let right_expr = self.parse_expr(prec);
        Expression::Infix(InfixExpr {
            left: Box::from(left_expr),
            right: Box::from(right_expr),
            op,
        })
    }

    fn parse_prefix_expr(&mut self) -> Expression {
        let op = match &self.cur_tok {
            Token::Operator(op) => Self::reg_op_to_pre_op(op),
            Token::ExclamMark => PrefixOp::Not,
            other => panic!("Expected operator, got: {other} instead"),
        };
        self.next_token();
        let val = Box::from(self.parse_expr(Precedence::Prefix));
        Expression::Prefix(PrefixExpr { op, val })
    }

    fn cur_tok_to_in_op(&self) -> InfixOp {
        match &self.cur_tok {
            Token::Operator(op) => Self::reg_op_to_in_op(op),
            _ => todo!(),
        }
    }

    fn reg_op_to_pre_op(op: &Operator) -> PrefixOp {
        match op {
            Operator::Plus => PrefixOp::Pos,
            Operator::Minus => PrefixOp::Neg,
            other => panic!("Cannot convert operator: {other} to pre op"),
        }
    }

    fn reg_op_to_in_op(op: &Operator) -> InfixOp {
        match op {
            Operator::Equals => InfixOp::Eq,
            Operator::NotEquals => InfixOp::NEq,
            Operator::Greater => InfixOp::GT,
            Operator::Lesser => InfixOp::LT,
            Operator::GreaterEquals => InfixOp::GTEq,
            Operator::LesserEquals => InfixOp::LTEq,
            Operator::Plus => InfixOp::Add,
            Operator::Minus => InfixOp::Sub,
            Operator::Asterisk => InfixOp::Mul,
            Operator::Slash => InfixOp::Div,
        }
    }

    /// First token needs to be the begin_token like `(` or `{` for example
    /// This function sets cur_tok to the end_tok
    fn parse_ident_list(&mut self, end_tok: Token) -> Vec<OptionallyTypedIdent> {
        if self.peek_tok == end_tok {
            self.next_token();
            return Vec::new();
        }

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

    /// First token needs to be the begin_token like `(` or `{` for example
    fn parse_raw_list(&mut self, end_tok: Token) -> Vec<Expression> {
        if self.peek_tok == Token::RParent {
            self.next_token();
            return Vec::new();
        }

        self.next_token();

        let mut items = Vec::new();

        let first_item = self.parse_expr(Precedence::Lowest);

        items.push(first_item);

        if self.peek_tok == Token::Comma {
            self.next_token();
        }

        while self.peek_tok != end_tok {
            self.next_token();
            let ident = self.parse_expr(Precedence::Lowest);
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

        self.next_token();
        while self.peek_tok != Token::RCurly {
            self.next_token();
            let stmt = self
                .parse_stmt()
                .expect("Found eof even though the blockstatement was not yet fully parsed");
            stmts.push(stmt);
            self.next_token();
        }
        self.next_token();
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

        let val = self.parse_expr(Precedence::Lowest);

        Statement::Variable(VarStmt {
            name: OptionallyTypedIdent { ident: name, _type },
            val,
            is_const,
        })
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

        let val = self.parse_expr(Precedence::Lowest);

        Statement::Variable(VarStmt {
            name: OptionallyTypedIdent { ident: name, _type },
            val,
            is_const,
        })
    }

    fn parse_call_expr(&mut self, func: Expression) -> Expression {
        let args = self.parse_raw_list(Token::RParent);
        Expression::Call(CallExpr {
            ident: Box::from(func),
            args,
        })
    }

    fn expect_peek(&self, expected: Token) {
        if self.peek_tok != expected {
            panic!("Expected: {}, received: {}", expected, self.peek_tok)
        }
    }

    fn peek_is_end(&self) -> bool {
        matches!(self.peek_tok, Token::Eol | Token::Eof)
    }

    pub fn next_token(&mut self) {
        swap(&mut self.cur_tok, &mut self.peek_tok);
        self.peek_tok = util::get_next_tok(self.lexer);
    }

    fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Assign => Precedence::Assign,
            Token::Operator(op) => match op {
                Operator::Equals | Operator::NotEquals => Precedence::Equals,
                Operator::Greater | Operator::Lesser => Precedence::LessGreater,
                Operator::GreaterEquals | Operator::LesserEquals => Precedence::LessGreaterOrEqual,
                Operator::Plus | Operator::Minus => Precedence::Sum,
                Operator::Asterisk | Operator::Slash => Precedence::Product,
            },
            Token::LParent => Precedence::Call,
            Token::LSquare => Precedence::Index,
            _ => Precedence::Lowest,
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
