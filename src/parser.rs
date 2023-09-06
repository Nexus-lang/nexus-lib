use std::process;

use crate::{
    ast::*,
    errors::*,
    lexer::Lexer,
    tokens::{Token, TokenType},
    util,
};
/// Parser struct containing
/// necessary info to
/// construct Evaluator
/// and keep track of errors
pub struct Parser {
    /// Reference to the lexer
    /// to access file and tokenstream
    lexer: Lexer,
    /// Stream constructed from lexer tokens
    token_stream: Vec<Token>,

    cur_token: Token,
    peek_token: Token,
    current_pos: usize,

    // Error handling
    errors: Vec<String>,

    /// Returns amount of lines in file
    /// to construct error message
    line_count: i32,
    // -------
}

// determines which operations has priority.
// E.g. 5 + 5 * 3, 5 + 15 = 20;
// We see that the product is higher than the sum.
const LOWEST: i8 = 0;
const EQUALS: i8 = 1; // ==
const LESSGREATER: i8 = 2; // > or <
const LESSGREATEREQUAL: i8 = 3; // >= or <=
const SUM: i8 = 4; // +
const PRODUCT: i8 = 5; // *
const PREFIX: i8 = 6; // -x, +x or !x
const CALL: i8 = 7; // amogus(x)
const CONVERSION: i8 = 8;

const EMPTY_EXPRESSION_STATEMENT: Statement = Statement::EXPRESSION(ExpressionStatement {
    expression: Expression::EMPTY,
});

impl Parser {
    /// Construct Parser from lexer
    pub fn new(lexer: &mut Lexer) -> Self {
        let token_stream: Vec<Token> = lexer.lex();
        return Parser {
            cur_token: token_stream[0].clone(),
            peek_token: token_stream[1].clone(),
            errors: vec![],
            current_pos: 0,
            lexer: lexer.clone(),
            line_count: 1,
            token_stream,
        };
    }

    /// Increment position in tokenstream
    /// and update cur- and peek_token
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

    /// "Main" function of the parser,
    /// constructs the ast
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();
            if statement != Statement::EMPTY {
                program.statements.push(statement);
            }
        }
        println!("Errors: {:?}", self.errors);

        program
    }

    /// returns statement
    /// depending on current token
    fn parse_statement(&mut self) -> Statement {
        match self.cur_token.token_type {
            TokenType::VAR => self.parse_vars(),
            TokenType::RETURN => self.parse_return_statement(),
            TokenType::CONST => self.parse_vars(),
            TokenType::LOCAL => self.parse_local_decl(),
            TokenType::ILLEGAL => {
                self.next_token();
                Statement::EMPTY
            }

            TokenType::EOL => {
                self.next_token();
                Statement::EMPTY
            }

            _ => {
                if self.cur_token_is(TokenType::IDENT)
                    && (self.peek_token_is(TokenType::VARASSIGN)
                        || self.peek_token_is(TokenType::CONSTASSIGN))
                {
                    return self.parse_quick_assign();
                }
                self.parse_expression_statement()
            }
        }
    }

    fn parse_local_decl(&mut self) -> Statement {
        if self.peek_token_is(TokenType::LOCAL) {
            self.throw_error(invalid_local_decl(&self.peek_token), true);
        }
        self.next_token();
        let left = self.parse_statement();
        if left == EMPTY_EXPRESSION_STATEMENT {
            self.throw_error(empty_local_decl(&left), true);
        }
        let statement = LocalStatement {
            left: Box::new(left),
        };
        Statement::LOCAL(statement)
    }

    /// parser var and const
    fn parse_vars(&mut self) -> Statement {
        let statement = match self.cur_token.token_type {
            TokenType::VAR => Statement::VAR(VarStatement {
                name: Identifier {
                    value: self.peek_token.literal.clone(),
                },
                value: {
                    // Identifier
                    self.next_token();
                    // assign
                    self.next_token();
                    // value
                    self.next_token();
                    let expression = self.parse_expression(LOWEST);

                    if expression == Expression::EMPTY {
                        self.throw_error(empty_variable_val(&expression), true);
                        Expression::EMPTY
                    } else {
                        expression
                    }
                },
            }),
            TokenType::CONST => Statement::CONST(ConstStatement {
                name: Identifier {
                    value: self.peek_token.literal.clone(),
                },
                value: {
                    // Identifier
                    self.next_token();
                    // assign
                    self.next_token();
                    // value
                    self.next_token();
                    let expression = self.parse_expression(LOWEST);

                    if expression == Expression::EMPTY {
                        self.throw_error(empty_variable_val(&expression), true);
                        Expression::EMPTY
                    } else {
                        expression
                    }
                },
            }),
            _ => panic!(),
        };

        // Go to EOL or semicolon
        self.continue_till_end();

        statement
    }

    /// parse varianles that are initialized with := or ::
    fn parse_quick_assign(&mut self) -> Statement {
        let statement = match self.peek_token.token_type {
            TokenType::VARASSIGN => Statement::VAR(VarStatement {
                name: Identifier {
                    value: self.cur_token.literal.clone(),
                },
                value: {
                    // Assign
                    self.next_token();
                    // value
                    self.next_token();
                    let expression = self.parse_expression(LOWEST);
                    if expression == Expression::EMPTY {
                        self.throw_error(empty_variable_val(&expression), true);
                        Expression::EMPTY
                    } else {
                        expression
                    }
                },
            }),
            TokenType::CONSTASSIGN => Statement::CONST(ConstStatement {
                name: Identifier {
                    value: self.cur_token.literal.clone(),
                },
                value: {
                    // Assign
                    self.next_token();
                    // value
                    self.next_token();
                    let expression = self.parse_expression(LOWEST);
                    if expression == Expression::EMPTY {
                        self.throw_error(empty_variable_val(&expression), true);
                        Expression::EMPTY
                    } else {
                        expression
                    }
                },
            }),
            _ => panic!(),
        };

        // Go to EOL or semicolon
        self.continue_till_end();

        statement
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.next_token();
        let statement = ReturnStatement {
            return_value: {
                let expression = self.parse_expression(LOWEST);
                if expression == Expression::EMPTY {
                    self.throw_error(empty_return_val(&expression), true);
                    Expression::EMPTY
                } else {
                    expression
                }
            },
        };

        // Skip expression and EOL
        self.continue_till_end();

        Statement::RETURN(statement)
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::RCURLY) && !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement();

            if statement != Statement::EMPTY
                && statement
                    != Statement::EXPRESSION(ExpressionStatement {
                        expression: Expression::EMPTY,
                    })
            {
                statements.push(statement);
            }
            self.next_token();
        }

        BlockStatement { statements }
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expression = self.parse_expression(LOWEST);
        let statement = ExpressionStatement { expression };
        self.next_token();
        Statement::EXPRESSION(statement)
    }

    /// Returns expression
    /// depending on current token
    fn parse_expression(&mut self, precedence: i8) -> Expression {
        let prefix = self.prefix_parse();
        if prefix == Expression::EMPTY {
            self.no_prefix_parse_error();
            return Expression::EMPTY;
        }
        let mut left_expression = prefix;

        while !self.peek_token_is_end() && precedence < self.peek_precedence() {
            self.next_token();
            left_expression = self.parse_infix_expression(left_expression.clone());
        }

        left_expression
    }

    fn parse_identifier(&self) -> Expression {
        Expression::IDENTIFIER(Identifier {
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_number_literal(&self) -> Expression {
        let err = format!("Cannot convert literal: {} to int", self.cur_token.literal);
        let literal = NumberLiteral {
            value: self.cur_token.literal.parse().expect(err.as_str()),
        };
        Expression::NUMBERLITERAL(literal)
    }

    fn parse_string_literal(&self) -> Expression {
        let literal = StringLiteral {
            value: self.cur_token.literal.clone(),
        };
        Expression::STRINGLITERAL(literal)
    }

    fn parse_if_expression(&mut self) -> Expression {
        self.next_token();
        let condition = Box::new(self.parse_expression(LOWEST));

        if condition == Box::from(Expression::EMPTY) {
            self.throw_error(empty_condition(&TokenType::IF, &condition), true);
            return Expression::EMPTY;
        }

        if !self.expect_peek(TokenType::LCURLY) {
            self.peek_error(TokenType::LCURLY);
            return Expression::EMPTY;
        }

        self.next_token();

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();

            alternative = self.parse_else_expression();
        }

        let expression = IfExpression {
            if_type: IfType::IF,
            condition,
            consequence,
            alternative,
        };

        Expression::IF(expression)
    }

    /// Responsible for parsing `else` and `else if`
    fn parse_else_expression(&mut self) -> Option<Box<IfExpression>> {
        match self.peek_token.token_type {
            TokenType::IF => {
                let if_type = IfType::ELSEIF;
                self.next_token();

                self.next_token();

                let condition = Box::new(self.parse_expression(LOWEST));

                if !self.expect_peek(TokenType::LCURLY) {
                    self.peek_error(TokenType::LCURLY);
                    return None;
                }

                self.next_token();

                let consequence = self.parse_block_statement();

                let mut alternative: Option<Box<IfExpression>> = None;

                if self.expect_peek(TokenType::ELSE) {
                    alternative = self.parse_else_expression();
                }

                let expression = Some(Box::new(IfExpression {
                    if_type,
                    condition,
                    consequence,
                    alternative,
                }));
                expression
            }
            TokenType::LCURLY => {
                let if_type = IfType::ELSE;

                let condition = Box::new(Expression::EMPTY);

                self.next_token();

                self.next_token();

                let consequence = self.parse_block_statement();

                let alternative: Option<Box<IfExpression>> = None;

                let expression = Some(Box::new(IfExpression {
                    if_type,
                    condition,
                    consequence,
                    alternative,
                }));
                expression
            }
            _ => {
                self.throw_error("Invalid token after else expression".to_string(), true);
                None
            }
        }
    }

    fn parse_while_expression(&mut self) -> Expression {
        self.next_token();
        let condition = Box::new(self.parse_expression(LOWEST));

        if condition == Box::from(Expression::EMPTY) {
            self.throw_error(empty_condition(&TokenType::WHILE, &condition), true);
            return Expression::EMPTY;
        }

        if !self.expect_peek(TokenType::LCURLY) {
            self.peek_error(TokenType::LCURLY);
            return Expression::EMPTY;
        }

        self.next_token();

        let consequence = self.parse_block_statement();

        let expression = WhileExpression {
            condition,
            consequence,
        };
        Expression::WHILE(expression)
    }

    fn parse_for_expression(&mut self) -> Expression {
        if !self.expect_peek(TokenType::IDENT) {
            self.peek_error(TokenType::IDENT);
            return Expression::EMPTY;
        }

        let ident = Identifier {
            value: self.cur_token.literal.clone(),
        };
        println!("{}", &self.cur_token.literal);

        if !self.expect_peek(TokenType::IN) {
            self.peek_error(TokenType::IN);
            return Expression::EMPTY;
        }
        self.next_token();

        let loop_list = Box::from(self.parse_expression(LOWEST));

        if !self.expect_peek(TokenType::LCURLY) {
            self.peek_error(TokenType::LCURLY);
            return Expression::EMPTY;
        }

        self.next_token();

        let consequence = self.parse_block_statement();

        let expression = ForExpression {
            ident,
            loop_list,
            consequence,
        };
        Expression::FOR(expression)
    }

    fn parse_grouped_expression(&mut self) -> Expression {
        self.next_token();

        let expression = self.parse_expression(LOWEST);

        if !self.expect_peek(TokenType::RPARENT) {
            self.peek_error(TokenType::RPARENT);
            return Expression::EMPTY;
        }

        expression
    }

    /// constructs prefix expression
    /// based on operator and
    /// expression affected by prefix
    fn parse_prefix_expression(&mut self) -> Expression {
        let operator = match self.cur_token.token_type {
            TokenType::PLUS => Operators::PLUS,
            TokenType::MINUS => Operators::MINUS,
            TokenType::BANG => Operators::BANG,
            _ => panic!("Invalid prefix ERROR. Please report"),
        };

        self.next_token();

        let right = self.parse_expression(PREFIX);

        Expression::PREFIX(PrefixExpression {
            right: Box::new(right),
            operator,
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let mut expression = InfixExpression {
            operator: self.get_operator(&self.cur_token),
            left: Box::new(left),
            right: Box::new(Expression::EMPTY),
        };
        let precedence = self.cur_precedence();
        self.next_token();
        expression.right = Box::new(self.parse_expression(precedence));
        Expression::INFIX(expression)
    }

    fn parse_boolean(&mut self) -> Expression {
        match self.cur_token.token_type {
            TokenType::TRUE => Expression::BOOLEAN(Boolean {
                bool_type: Booleans::TRUE,
            }),
            TokenType::FALSE => Expression::BOOLEAN(Boolean {
                bool_type: Booleans::FALSE,
            }),
            _ => Expression::EMPTY,
        }
    }

    fn get_operator(&self, token: &Token) -> Operators {
        match token.token_type {
            TokenType::PLUS => Operators::PLUS,
            TokenType::MINUS => Operators::MINUS,
            TokenType::BANG => Operators::BANG,
            TokenType::MULTIPLY => Operators::MULTIPLY,
            TokenType::DIVIDE => Operators::DIVIDE,
            TokenType::EQUAL => Operators::EQUAL,
            TokenType::NOTEQUAL => Operators::NOTEQUAL,
            TokenType::GREATERTHAN => Operators::GREATTHAN,
            TokenType::LESSTHAN => Operators::LESSTHAN,
            TokenType::GREATEROREQUALTHAN => Operators::GREATOREQUAL,
            TokenType::LESSOREQUALTHAN => Operators::LESSOREQUAL,
            TokenType::AS => Operators::AS,
            _ => Operators::ILLEGAL,
        }
    }

    fn get_precedence(&self, token: &Token) -> i8 {
        match token.token_type {
            TokenType::EQUAL => EQUALS,
            TokenType::NOTEQUAL => EQUALS,
            TokenType::LESSTHAN => LESSGREATER,
            TokenType::GREATERTHAN => LESSGREATER,
            TokenType::PLUS => SUM,
            TokenType::MINUS => SUM,
            TokenType::DIVIDE => PRODUCT,
            TokenType::MULTIPLY => PRODUCT,
            TokenType::AS => CONVERSION,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> i8 {
        self.get_precedence(&self.cur_token)
    }

    fn peek_precedence(&self) -> i8 {
        self.get_precedence(&self.peek_token)
    }

    /// return error when prefix is missing
    fn no_prefix_parse_error(&mut self) {
        self.errors.push(format!(
            "no prefix parse function for {:?} found",
            self.cur_token.token_type
        ))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// returns true if peek token is eol or eof
    fn peek_token_is_end(&self) -> bool {
        match self.peek_token.token_type {
            TokenType::EOL | TokenType::EOF | TokenType::SEMICOLON => true,
            _ => false,
        }
    }

    /// evaluates whether peek expectation is fulfilled
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    /// returns error based on expected token type
    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, found {:?} instead.",
            token_type, self.peek_token.token_type,
        );
        self.throw_error(msg, true);
    }

    /// appends error to [`Parser::errors`]
    /// if exit is set to `true`
    /// it will stop the program
    fn throw_error(&mut self, message: String, exit: bool) {
        let msg = format!(
            "{} error at: {}:{}:{}",
            message,
            self.lexer.input.file_path,
            self.line_count,
            self.peek_token.cur_pos + 1,
        );
        self.errors.push(msg);
        if exit {
            println!("{:?}", self.errors);
            println!("PRESS ANY KEY TO EXIT");
            util::input();
            process::exit(1);
        }
    }

    fn prefix_parse(&mut self) -> Expression {
        match self.cur_token.token_type {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::NUMBER => self.parse_number_literal(),
            TokenType::STRING => self.parse_string_literal(),
            TokenType::LPARENT => self.parse_grouped_expression(),
            /*
            TokenType::FUNC => self.parse_function_literal(),
            TokenType::LSQUAREBRAC => self.parse_list_literal(),
            TokenType::LCURLY => self.parse_hash_literal(),
            */
            TokenType::IF => self.parse_if_expression(),
            TokenType::WHILE => self.parse_while_expression(),
            TokenType::FOR => self.parse_for_expression(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean(),
            TokenType::BANG | TokenType::MINUS | TokenType::PLUS => self.parse_prefix_expression(),
            _ => Expression::EMPTY,
        }
    }

    fn continue_till_end(&mut self) {
        while !self.cur_token_is(TokenType::EOL) && !self.cur_token_is(TokenType::EOF) {
            self.next_token();
        }
    }
}
