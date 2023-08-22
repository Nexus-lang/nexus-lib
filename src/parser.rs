use std::process;

use crate::{
    ast::*,
    lexer::Lexer,
    tokens::{Token, TokenType},
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

/// TODO: add documentation once fully implemented
#[allow(dead_code)] // TODO: remove this once all types are used
enum Precedences {
    LOWEST,
    EQUALS,           // ==
    LESSGREATER,      // > or <
    LESSGREATEREQUAL, // >= or <=
    SUM,              // +
    PRODUCT,          // *
    PREFIX,           // -x, +x or !x
    CALL,             // amogus(x)
}

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
        println!("{:?}", self.errors);

        program
    }

    /// returns statement
    /// depending on current token
    fn parse_statement(&mut self) -> Statement {
        match self.cur_token.token_type {
            TokenType::VAR => self.parse_var_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            // might have to be improved in the future
            TokenType::ILLEGAL => {
                self.next_token();
                Statement::EMPTY
            }

            TokenType::EOL => {
                self.next_token();
                Statement::EMPTY
            }

            _ => self.parse_expression_statement(),
        }
    }

    fn parse_var_statement(&mut self) -> Statement {
        let mut statement = VarStatement {
            name: Identifier {
                value: "".to_string(),
            },
            value: Expression::EMPTY,
        };
        if !self.expect_peek(TokenType::IDENT) {
            return Statement::EMPTY;
        }

        statement.name = Identifier {
            value: self.cur_token.clone().literal,
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return Statement::EMPTY;
        }

        while !self.cur_token_is(TokenType::EOL) && !self.cur_token_is(TokenType::EOF) {
            self.next_token();
        }

        // TODO: Expression parsing

        Statement::VAR(statement)
    }

    fn parse_return_statement(&mut self) -> Statement {
        let statement = ReturnStatement {
            return_value: Expression::EMPTY,
        };

        // Skip expression and EOL
        while !self.cur_token_is(TokenType::EOL) && !self.cur_token_is(TokenType::EOF) {
            self.next_token();
        }

        // TODO: Expression parsing

        Statement::RETURN(statement)
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expression = self.parse_expression(Precedences::LOWEST);
        let statement = ExpressionStatement { expression };
        self.next_token();
        Statement::EXPRESSION(statement)
    }

    /// Returns expression
    /// depending on current token
    fn parse_expression(&mut self, _precedence: Precedences) -> Expression {
        let prefix = self.prefix_parse();
        if prefix == Expression::EMPTY {
            self.no_prefix_parse_error();
            return Expression::EMPTY;
        }
        let left_expression = prefix;
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

    /// constructs prefix expression
    /// based on operator and
    /// expression affected by prefix
    fn parse_prefix_expression(&mut self) -> Expression {
        let operator = self.cur_token.literal.to_owned();

        self.next_token();

        let right = self.parse_expression(Precedences::PREFIX);

        Expression::PREFIX(PrefixExpression {
            right: Box::new(right),
            operator,
        })
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
            "expected next token to be {:?}, found {:?} instead. error at: {}:{}:{}",
            token_type,
            self.peek_token.token_type,
            self.lexer.input.file_path,
            self.line_count,
            self.peek_token.cur_pos + 1,
        );
        self.throw_error(msg, false);
    }

    /// appends error to [`Parser::errors`]
    /// if exit is set to `true`
    /// it will stop the program
    fn throw_error(&mut self, message: String, exit: bool) {
        self.errors.push(message);
        if exit {
            println!("{:?}", self.errors);
            process::exit(1);
        }
    }

    fn prefix_parse(&mut self) -> Expression {
        match self.cur_token.token_type {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::NUMBER => self.parse_number_literal(),
            TokenType::STRING => self.parse_string_literal(),
            /*
            TokenType::FUNC => self.parse_function_literal(),
            TokenType::LPARENT => self.parse_grouped_expression(),
            TokenType::LSQUAREBRAC => self.parse_list_literal(),
            TokenType::LCURLY => self.parse_hash_literal(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean(),
            */
            TokenType::BANG | TokenType::MINUS | TokenType::PLUS => self.parse_prefix_expression(),
            _ => Expression::EMPTY,
        }
    }
}
