use crate::{
    tokens::{Token, TokenType},
    util::{FileHandler, FirstAsChar},
};

/// Same as tokens.push() but reduces boilerplate
macro_rules! push_token {
    ($tokens:expr, $variant:path, $literal:expr, $cur_pos:expr) => {
        $tokens.push(Token::new($variant, $literal, $cur_pos))
    };
    ($tokens:expr, $variant:path, $cur_pos:expr) => {
        push_token!($tokens, $variant, $variant.literal(), $cur_pos)
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
    pub fn lex(&mut self, alt_lex_string: Option<String>) -> Vec<Token> {
        let input_chars: Vec<char> = match alt_lex_string {
            Some(alt) => {
                alt.chars().collect()
            },
            None => self.input.file_content.chars().collect(),
        };
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

                    push_token!(tokens, TokenType::NUMBER, identifier, self.current_pos_line);
                }
                c if c.is_alphabetic() || c == '_' => {self.lex_ident(&mut tokens, &input_chars);},
                c if c == TokenType::QUOTMARK.first_as_char()
                    || c == TokenType::APOSTROPHE.first_as_char() =>
                {
                    self.lex_string(&mut tokens, &input_chars, c);
                }
                c if c.is_ascii()
                    && !c.is_alphanumeric()
                    && c != TokenType::QUOTMARK.first_as_char()
                    && c != TokenType::APOSTROPHE.first_as_char() =>
                {
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
                            } else if input_chars[self.current_pos - 1]
                                != TokenType::COLON.first_as_char()
                            {
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
                            if input_chars[self.current_pos + 1] == TokenType::COLON.first_as_char()
                            {
                                push_token!(tokens, TokenType::CONSTASSIGN, self.current_pos_line);
                            } else if input_chars[self.current_pos + 1]
                                == TokenType::ASSIGN.first_as_char()
                            {
                                push_token!(tokens, TokenType::VARASSIGN, self.current_pos_line);
                            } else if input_chars[self.current_pos - 1]
                                != TokenType::COLON.first_as_char()
                            {
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
                            push_token!(tokens, TokenType::ILLEGAL, c.to_string(), self.current_pos_line);
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

        tokens
    }

    fn lex_string(
        &mut self,
        tokens: &mut Vec<Token>,
        input_chars: &Vec<char>,
        c /* current char */: char,
    ) {
        let mut identifier = String::new();

        let mut next_pos = self.current_pos + 1;
        push_token!(tokens, TokenType::STRINGB, self.current_pos_line);
        while next_pos < input_chars.len() {
            // check whether first character is apostrophe or quotation mark
            // and end string depending on that ;)
            if (c == TokenType::QUOTMARK.first_as_char()
                && input_chars[next_pos] != TokenType::QUOTMARK.first_as_char())
                || (c == TokenType::APOSTROPHE.first_as_char()
                    && input_chars[next_pos] != TokenType::APOSTROPHE.first_as_char())
            {
                // check if reference is passed into string
                // Reference example: "Hello, {name()}"
                if input_chars[next_pos] == TokenType::LCURLY.first_as_char() {
                    push_token!(tokens, TokenType::STRING, identifier.clone(), self.current_pos_line);
                    identifier = String::new();
                    next_pos += 1;
                    // update string lexing position after string reference has been tokenized
                    next_pos = self.lex_string_ref(tokens, input_chars, next_pos);
                    next_pos += 1;
                } else {
                    identifier.push(input_chars[next_pos]);
                    next_pos += 1;
                }
            } else {
                break;
            }
        }
        push_token!(tokens, TokenType::STRING, identifier.clone(), self.current_pos_line);
        push_token!(tokens, TokenType::STRINGE, self.current_pos_line);
        next_pos += 1;
        self.current_pos = next_pos;
    }

    fn lex_ident(&mut self, tokens: &mut Vec<Token>, input_chars: &Vec<char>) -> Vec<Token> {
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
                push_token!(tokens, TokenType::IDENT, identifier, self.current_pos_line);
            }
        }
        tokens.to_vec()
    }

    /// lexes a string reference. Built for lex string. Do not use outside!
    /// `Returns: `
    fn lex_string_ref(
        &mut self,
        tokens: &mut Vec<Token>,
        input_chars: &Vec<char>,
        next_pos: usize,
    ) -> usize {
        let mut mut_next_pos = next_pos;
        let mut identifier = String::new();
        push_token!(tokens, TokenType::STRINGREFB, self.current_pos_line);

        while mut_next_pos < input_chars.len()
            && input_chars[mut_next_pos] != TokenType::RCURLY.first_as_char()
        {
            identifier.push(input_chars[mut_next_pos]);
            mut_next_pos += 1;
        }

        let mut reference = self.lex(Some(identifier));
        reference.pop();

        self.current_pos = mut_next_pos;

        reference.iter().for_each(|t| tokens.push(t.to_owned()));

        push_token!(tokens, TokenType::STRINGREFE, self.current_pos_line);

        mut_next_pos
    }
}
