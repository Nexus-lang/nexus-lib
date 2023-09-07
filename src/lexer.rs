use crate::{
    tokens::{Token, TokenType},
    util::{FileHandler, ToChar},
};

/// Same as tokens.push() but reduces boilerplate
macro_rules! push_token {
    ($tokens:expr, $variant:path, $cur_pos:expr) => {
        $tokens.push(Token::new($variant, $variant.literal(), $cur_pos))
    };
}

/// Lexer struct containing
/// necessary info to
/// construct the parser
#[derive(Clone)]
pub struct Lexer {
    pub input: FileHandler,
    pub current_pos: usize,
    /// Returns the current pos.
    ///
    /// Resets the current pos
    /// whenever it encounters a new line
    pub current_pos_line: usize,
    ch: char,
}

impl Lexer {
    /// Constructs lexer from FileHandler
    pub fn new(input: FileHandler) -> Self {
        let input_chars: Vec<char> = input.file_content.chars().collect();
        if input_chars.len() > 0 {
            return Lexer {
                input: input,
                current_pos: 0,
                current_pos_line: 1,
                ch: input_chars[0],
            };
        } else {
            todo!("Handle empty files")
        }
    }

    /// Tokenizes the [`Lexer::input`]
    pub fn lex(&mut self) -> Vec<Token> {
        let input_chars: Vec<char> = self.input.file_content.chars().collect();
        let mut tokens: Vec<Token> = Vec::new();

        self.current_pos = 0;
        self.current_pos_line = 1;
        while self.current_pos < input_chars.len() {
            self.ch = input_chars[self.current_pos];

            match self.ch {
                // handle new lines
                c if c == '\n' => {
                    push_token!(tokens, TokenType::EOL, self.current_pos_line);
                    self.current_pos += 1;
                    self.current_pos_line = 0;
                }

                // skip white spaces
                c if c.is_whitespace() && c != '\n' => {
                    self.current_pos += 1;
                    self.current_pos_line += 1;
                }

                // construct number from chained digits.
                // also has support for hexadecimal, octal and binary numbers
                c if c.is_numeric() => {
                    let mut identifier = String::new();

                    while self.current_pos < input_chars.len()
                        && (input_chars[self.current_pos].is_numeric()
                        // hexadec, octal, bin
                            || input_chars[self.current_pos] == 'x'
                            || input_chars[self.current_pos] == 'b'
                            || input_chars[self.current_pos] == 'o')
                    {
                        identifier.push(input_chars[self.current_pos]);
                        self.current_pos += 1;
                        self.current_pos_line += 1;
                    }

                    tokens.push(Token::new(
                        TokenType::NUMBER,
                        identifier,
                        self.current_pos_line,
                    ));
                }

                // handle
                c if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    identifier.push(self.ch);

                    let mut next_pos = self.current_pos + 1;

                    while next_pos < input_chars.len()
                        && (input_chars[next_pos].is_alphanumeric() || input_chars[next_pos] == '_')
                    {
                        identifier.push(input_chars[next_pos]);
                        next_pos += 1;
                        self.current_pos_line += 1;
                    }

                    self.current_pos = next_pos;

                    match &identifier {
                        i if *i == TokenType::DEP.literal() => {
                            push_token!(tokens, TokenType::DEP, self.current_pos_line);
                        }
                        i if *i == TokenType::USE.literal() => {
                            push_token!(tokens, TokenType::USE, self.current_pos_line);
                        }
                        i if *i == TokenType::TAG.literal() => {
                            push_token!(tokens, TokenType::TAG, self.current_pos_line);
                        }
                        i if *i == TokenType::STRUCT.literal() => {
                            push_token!(tokens, TokenType::STRUCT, self.current_pos_line);
                        }
                        i if *i == TokenType::ENUM.literal() => {
                            push_token!(tokens, TokenType::ENUM, self.current_pos_line);
                        }
                        i if *i == TokenType::VAR.literal() => {
                            push_token!(tokens, TokenType::VAR, self.current_pos_line);
                        }
                        i if *i == TokenType::CONST.literal() => {
                            push_token!(tokens, TokenType::CONST, self.current_pos_line)
                        }
                        i if *i == TokenType::FUNC.literal() => {
                            push_token!(tokens, TokenType::FUNC, self.current_pos_line);
                        }
                        i if *i == TokenType::RETURN.literal() => {
                            push_token!(tokens, TokenType::RETURN, self.current_pos_line);
                        }
                        i if *i == TokenType::FOR.literal() => {
                            push_token!(tokens, TokenType::FOR, self.current_pos_line);
                        }
                        i if *i == TokenType::WHILE.literal() => {
                            push_token!(tokens, TokenType::WHILE, self.current_pos_line);
                        }
                        i if *i == TokenType::IF.literal() => {
                            push_token!(tokens, TokenType::IF, self.current_pos_line);
                        }
                        i if *i == TokenType::WHEN.literal() => {
                            push_token!(tokens, TokenType::WHEN, self.current_pos_line);
                        }
                        i if *i == TokenType::ELSE.literal() => {
                            push_token!(tokens, TokenType::ELSE, self.current_pos_line);
                        }
                        i if *i == TokenType::TRY.literal() => {
                            push_token!(tokens, TokenType::TRY, self.current_pos_line);
                        }
                        i if *i == TokenType::CATCH.literal() => {
                            push_token!(tokens, TokenType::CATCH, self.current_pos_line);
                        }

                        i if *i == TokenType::LOCAL.literal() => {
                            push_token!(tokens, TokenType::LOCAL, self.current_pos_line);
                        }
                        i if *i == TokenType::TRUE.literal() => {
                            push_token!(tokens, TokenType::TRUE, self.current_pos_line);
                        }
                        i if *i == TokenType::FALSE.literal() => {
                            push_token!(tokens, TokenType::FALSE, self.current_pos_line);
                        }
                        i if *i == TokenType::NONE.literal() => {
                            push_token!(tokens, TokenType::NONE, self.current_pos_line);
                        }

                        i if *i == TokenType::IN.literal() => {
                            push_token!(tokens, TokenType::IN, self.current_pos_line);
                        }
                        i if *i == TokenType::AS.literal() => {
                            push_token!(tokens, TokenType::AS, self.current_pos_line);
                        }
                        i if *i == TokenType::AND.literal() => {
                            push_token!(tokens, TokenType::AND, self.current_pos_line);
                        }
                        i if *i == TokenType::OR.literal() => {
                            push_token!(tokens, TokenType::OR, self.current_pos_line);
                        }

                        _ => {
                            tokens.push(Token::new(
                                TokenType::IDENT,
                                identifier.to_owned(),
                                self.current_pos_line,
                            ));
                        }
                    }
                }
                c if c == TokenType::QUOTMARK.to_char() || c == TokenType::APOSTROPHE.to_char() => {
                    let mut identifier = String::new();

                    let mut next_pos = self.current_pos + 1;
                    while next_pos < input_chars.len() {
                        if c == TokenType::QUOTMARK.to_char()
                            && input_chars[next_pos] != TokenType::QUOTMARK.to_char()
                            || c == TokenType::APOSTROPHE.to_char()
                                && input_chars[next_pos] != TokenType::APOSTROPHE.to_char()
                        {
                            identifier.push(input_chars[next_pos]);
                            next_pos += 1;
                            self.current_pos_line += 1;
                        } else {
                            next_pos += 1;
                            break;
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::STRING,
                        identifier,
                        self.current_pos_line,
                    ));

                    self.current_pos = next_pos;
                }
                c if c.is_ascii() && !c.is_alphanumeric() => {
                    match c.to_string() {
                        c if c == TokenType::ASSIGN.literal() => {
                            let equal_chars: Vec<char> =
                                TokenType::EQUAL.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == equal_chars[1]
                            {
                                push_token!(tokens, TokenType::EQUAL, self.current_pos_line);
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else if input_chars[self.current_pos - 1] != TokenType::COLON.to_char() {
                                push_token!(tokens, TokenType::ASSIGN, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::MINUS.literal() => {
                            let arrow_chars: Vec<char> =
                                TokenType::ARROW.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == arrow_chars[1]
                            {
                                push_token!(tokens, TokenType::ARROW, self.current_pos_line);
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else {
                                push_token!(tokens, TokenType::MINUS, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::GREATERTHAN.literal() => {
                            let greaterthan_chars: Vec<char> =
                                TokenType::GREATEROREQUALTHAN.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == greaterthan_chars[1]
                            {
                                push_token!(
                                    tokens,
                                    TokenType::GREATEROREQUALTHAN,
                                    self.current_pos_line
                                );
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else {
                                push_token!(tokens, TokenType::GREATERTHAN, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::LESSTHAN.literal() => {
                            let lessthan_chars: Vec<char> =
                                TokenType::LESSOREQUALTHAN.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == lessthan_chars[1]
                            {
                                push_token!(
                                    tokens,
                                    TokenType::LESSOREQUALTHAN,
                                    self.current_pos_line
                                );
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else {
                                push_token!(tokens, TokenType::LESSTHAN, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::BANG.literal() => {
                            let notequal_chars: Vec<char> =
                                TokenType::NOTEQUAL.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == notequal_chars[1]
                            {
                                push_token!(tokens, TokenType::NOTEQUAL, self.current_pos_line);
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else {
                                push_token!(tokens, TokenType::BANG, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::DIVIDE.literal() => {
                            let comment_chars: Vec<char> =
                                TokenType::COMMENT.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len()
                                && input_chars[self.current_pos + 1] == comment_chars[1]
                            {
                                push_token!(tokens, TokenType::COMMENT, self.current_pos_line);
                                self.current_pos += 1;
                                self.current_pos_line += 1;
                            } else {
                                push_token!(tokens, TokenType::DIVIDE, self.current_pos_line);
                            }
                        }
                        c if c == TokenType::COLON.literal() => {
                            if input_chars[self.current_pos + 1] == TokenType::COLON.to_char() {
                                push_token!(tokens, TokenType::CONSTASSIGN, self.current_pos_line);
                            } else if input_chars[self.current_pos + 1] == TokenType::ASSIGN.to_char() {
                                push_token!(tokens, TokenType::VARASSIGN, self.current_pos_line);
                            } else if input_chars[self.current_pos - 1] != TokenType::COLON.to_char() {
                                push_token!(tokens, TokenType::COLON, self.current_pos_line)
                            }
                        }
                        c if c == TokenType::COMMA.literal() => {
                            push_token!(tokens, TokenType::COMMA, self.current_pos_line)
                        }
                        c if c == TokenType::DOT.literal() => {
                            push_token!(tokens, TokenType::DOT, self.current_pos_line)
                        }
                        c if c == TokenType::PLUS.literal() => {
                            push_token!(tokens, TokenType::PLUS, self.current_pos_line)
                        }
                        c if c == TokenType::MULTIPLY.literal() => {
                            push_token!(tokens, TokenType::MULTIPLY, self.current_pos_line)
                        }
                        c if c == TokenType::LCURLY.literal() => {
                            push_token!(tokens, TokenType::LCURLY, self.current_pos_line)
                        }
                        c if c == TokenType::RCURLY.literal() => {
                            push_token!(tokens, TokenType::RCURLY, self.current_pos_line)
                        }
                        c if c == TokenType::LPARENT.literal() => {
                            push_token!(tokens, TokenType::LPARENT, self.current_pos_line)
                        }
                        c if c == TokenType::RPARENT.literal() => {
                            push_token!(tokens, TokenType::RPARENT, self.current_pos_line)
                        }
                        c if c == TokenType::LSQUAREBRAC.literal() => {
                            push_token!(tokens, TokenType::LSQUAREBRAC, self.current_pos_line)
                        }
                        c if c == TokenType::RSQUAREBRAC.literal() => {
                            push_token!(tokens, TokenType::RSQUAREBRAC, self.current_pos_line)
                        }
                        c if c == TokenType::SEMICOLON.literal() => {
                            push_token!(tokens, TokenType::SEMICOLON, self.current_pos_line)
                        }
                        c if c == TokenType::ANNOTATION.literal() => {
                            push_token!(tokens, TokenType::ANNOTATION, self.current_pos_line)
                        }
                        _ => {
                            tokens.push(Token::new(
                                TokenType::ILLEGAL,
                                c.to_string(),
                                self.current_pos_line,
                            ));
                        }
                    }
                    self.current_pos += 1;
                    self.current_pos_line += 1;
                }
                _ => {
                    push_token!(tokens, TokenType::ILLEGAL, self.current_pos_line);
                    self.current_pos += 1;
                    self.current_pos_line += 1;
                }
            }
        }
        push_token!(tokens, TokenType::EOF, self.current_pos_line);
        let test = &tokens[tokens.len() - 2];
        println!("{:?}", test);

        tokens
    }
}
