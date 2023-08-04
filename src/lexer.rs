use crate::tokens::{Token, TokenType};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()))
    };
}

pub struct Lexer {
    input: String,

    current_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input_chars: Vec<char> = input.chars().collect();
        return Lexer {
            input: input,
            current_pos: 0,
            ch: input_chars[0],
        };
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let input_chars: Vec<char> = self.input.chars().collect();
        let mut tokens: Vec<Token> = Vec::new();

        self.current_pos = 0;
        while self.current_pos < input_chars.len() {
            self.ch = input_chars[self.current_pos];

            match self.ch {
                // new lines
                c if c == '\n' => {
                    push_token!(tokens, TokenType::EOL);
                    self.current_pos += 1;
                }
                // skipping white spaces
                c if c.is_whitespace() && c != '\n' => {
                    self.current_pos += 1;
                }
                c if c.is_numeric() => {
                    let mut identifier = String::new();

                    while self.current_pos < input_chars.len()
                        && (input_chars[self.current_pos].is_numeric()
                            || input_chars[self.current_pos] == 'x'
                            || input_chars[self.current_pos] == 'b'
                            || input_chars[self.current_pos] == 'o')
                    {
                        identifier.push(input_chars[self.current_pos]);
                        self.current_pos += 1;
                    }

                    tokens.push(Token(TokenType::NUMBER, identifier));
                }

                c if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    identifier.push(self.ch);

                    let mut next_pos = self.current_pos + 1;

                    if input_chars[self.current_pos] == 'c' && input_chars[next_pos] == ':' {
                        identifier.push(input_chars[next_pos])
                    }

                    while next_pos < input_chars.len()
                        && (input_chars[next_pos].is_alphanumeric() || input_chars[next_pos] == '_')
                    {
                        identifier.push(input_chars[next_pos]);
                        next_pos += 1;
                    }

                    self.current_pos = next_pos;

                    match identifier.clone() {
                        i if i == TokenType::VAR.literal() => {
                            push_token!(tokens, TokenType::VAR);
                        }
                        i if i == TokenType::CONST.literal() => {
                            push_token!(tokens, TokenType::CONST)
                        }
                        i if i == TokenType::IF.literal() => {
                            push_token!(tokens, TokenType::IF);
                        }
                        i if i == TokenType::ELSE.literal() => {
                            push_token!(tokens, TokenType::ELSE);
                        }
                        i if i == TokenType::FOR.literal() => {
                            push_token!(tokens, TokenType::FOR);
                        }
                        i if i == TokenType::WHILE.literal() => {
                            push_token!(tokens, TokenType::WHILE);
                        }
                        i if i == TokenType::FUNC.literal() => {
                            push_token!(tokens, TokenType::FUNC);
                        }
                        i if i == TokenType::WHEN.literal() => {
                            push_token!(tokens, TokenType::WHEN);
                        }
                        i if i == TokenType::USE.literal() => {
                            push_token!(tokens, TokenType::USE);
                        }
                        i if i == TokenType::OTHER.literal() => {
                            push_token!(tokens, TokenType::OTHER);
                        }
                        i if i == TokenType::IN.literal() => {
                            push_token!(tokens, TokenType::IN);
                        }

                        // Literals
                        i if i == TokenType::TRUE.literal() => {
                            push_token!(tokens, TokenType::TRUE);
                        }
                        i if i == TokenType::FALSE.literal() => {
                            push_token!(tokens, TokenType::FALSE);
                        }

                        // Comparison
                        i if i == TokenType::AND.literal() => {
                            push_token!(tokens, TokenType::AND);
                        }
                        i if i == TokenType::OR.literal() => {
                            push_token!(tokens, TokenType::OR);
                        }

                        // Data Structures
                        i if i == TokenType::STRUCT.literal() => {
                            push_token!(tokens, TokenType::STRUCT);
                        }
                        i if i == TokenType::ENUM.literal() => {
                            push_token!(tokens, TokenType::ENUM);
                        }
                        i if i == TokenType::CONSTASSIGN.literal() => {
                            tokens.push(Token(TokenType::CONSTASSIGN, identifier));
                        }
                        _ => {
                            tokens.push(Token(TokenType::IDENT, identifier));
                        }
                    }
                }
                c if c == '"' || c == '\"' => {
                    let mut identifier = String::new();
                    if c == '"' { 
                        push_token!(tokens, TokenType::QUOTMARK)
                    } else {
                        push_token!(tokens, TokenType::APOSTROPHE)
                    }

                    let mut next_pos = self.current_pos + 1;
                    while next_pos < input_chars.len() {
                        if c == '"' && input_chars[next_pos] != '"'
                            || c == '\'' && input_chars[next_pos] != '\''
                        {
                            identifier.push(input_chars[next_pos]);
                            next_pos += 1;
                        } else {
                            next_pos += 1;
                            break;
                        }
                    }

                    tokens.push(Token(TokenType::STRING, identifier));
                    
                    if c == '"' { 
                        push_token!(tokens, TokenType::QUOTMARK)
                    } else {
                        push_token!(tokens, TokenType::APOSTROPHE)
                    }

                    self.current_pos = next_pos;
                    println!("{}", c);
                }
                c if c.is_ascii() && !c.is_alphanumeric() => {
                    match c.to_string() {
                        c if c == TokenType::ASSIGN.literal() => {
                            let equal_chars: Vec<char> = TokenType::EQUAL.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == equal_chars[1] {
                                push_token!(tokens, TokenType::EQUAL);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::ASSIGN);
                            }
                        }
                        c if c == TokenType::MINUS.literal() => {
                            let arrow_chars: Vec<char> = TokenType::ARROW.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == arrow_chars[1] {
                                push_token!(tokens, TokenType::ARROW);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::MINUS);
                            }
                        }
                        c if c == TokenType::GREATERTHAN.literal() => {
                            let greaterthan_chars: Vec<char> = TokenType::GREATEROREQUALTHAN.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == greaterthan_chars[1] {
                                push_token!(tokens, TokenType::GREATEROREQUALTHAN);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::GREATERTHAN);
                            }
                        }
                        c if c == TokenType::LESSTHAN.literal() => {
                            let lessthan_chars: Vec<char> = TokenType::LESSOREQUALTHAN.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == lessthan_chars[1] {
                                push_token!(tokens, TokenType::LESSOREQUALTHAN);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::LESSTHAN);
                            }
                        }
                        c if c == TokenType::EXCLAMMARK.literal() => {
                            let notequal_chars: Vec<char> = TokenType::NOTEQUAL.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == notequal_chars[1] {
                                push_token!(tokens, TokenType::NOTEQUAL);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::EXCLAMMARK);
                            }
                        }
                        c if c == TokenType::DIVIDE.literal() => {
                            let comment_chars: Vec<char> = TokenType::COMMENT.literal().chars().collect();
                            if self.current_pos + 1 < input_chars.len() && input_chars[self.current_pos + 1] == comment_chars[1] {
                                push_token!(tokens, TokenType::COMMENT);
                                self.current_pos += 1;
                            } else {
                                push_token!(tokens, TokenType::DIVIDE);
                            }
                        }
                        c if c == TokenType::COLON.literal() => {
                            if input_chars[self.current_pos - 1] != 'c' {
                                push_token!(tokens, TokenType::COLON);
                            }
                        }
                        c if c == TokenType::COMMA.literal() => {
                            push_token!(tokens, TokenType::COMMA)
                        }
                        c if c == TokenType::DOT.literal() => {
                            push_token!(tokens, TokenType::DOT)
                        }
                        c if c == TokenType::PLUS.literal() => {
                            push_token!(tokens, TokenType::PLUS)
                        }
                        c if c == TokenType::MULTIPLY.literal() => {
                            push_token!(tokens, TokenType::MULTIPLY)
                        }
                        c if c == TokenType::LCURLY.literal() => {
                            push_token!(tokens, TokenType::LCURLY)
                        }
                        c if c == TokenType::RCURLY.literal() => {
                            push_token!(tokens, TokenType::RCURLY)
                        }
                        c if c == TokenType::LPARENT.literal() => {
                            push_token!(tokens, TokenType::LPARENT)
                        }
                        c if c == TokenType::RPARENT.literal() => {
                            push_token!(tokens, TokenType::RPARENT)
                        }
                        c if c == TokenType::LSQUAREBRAC.literal() => {
                            push_token!(tokens, TokenType::LSQUAREBRAC)
                        }
                        c if c == TokenType::RSQUAREBRAC.literal() => {
                            push_token!(tokens, TokenType::RSQUAREBRAC)
                        }
                        _ => {
                            tokens.push(Token(TokenType::ILLEGAL, c.to_string()));
                        }
                    }
                    self.current_pos += 1;
                }
                _ => {
                    println!("{}", self.ch);
                    self.current_pos += 1;
                }
            }
        }

        tokens
    }
}
/*
fn is_utf8_and_not_ascii(c: char) -> bool {
    let s = c.to_string();
    let bytes = s.as_bytes();

    // Check if the bytes form a valid UTF-8 string

    if !s.is_ascii() {
        if let Ok(_) = std::str::from_utf8(bytes) {
            return true;
        } else {
            return false;
        }
    }
    false
}
*/
