use crate::tokens::{Token, TokenTypes};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()))
    };
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_pos = 0;
    let input_stream: Vec<char> = input.chars().collect();

    while current_pos < input.len() {
        let char = input_stream[current_pos];

        match char {
            // Skip newlines
            c if c.is_whitespace() && c != '\n' => {
                current_pos += 1;
            }
            '\n' => {
                push_token!(tokens, TokenTypes::EOL);
                current_pos += 1;
            }

            // Check keywords
            // Checks if first char is a letter
            c if c.is_alphabetic() => {
                let mut identifier: String = String::new();
                while current_pos < input.len() && input_stream[current_pos].is_alphanumeric() && !input_stream[current_pos].is_whitespace() {
                    identifier.push(input_stream[current_pos]);
                    current_pos += 1;
                }

                match identifier {
                    // Keywords
                    i if i == TokenTypes::VAR.literal() => {
                        push_token!(tokens, TokenTypes::VAR);
                    }
                    i if i == TokenTypes::IF.literal() => {
                        push_token!(tokens, TokenTypes::IF);
                    }
                    i if i == TokenTypes::ELSE.literal() => {
                        push_token!(tokens, TokenTypes::ELSE);
                    }
                    i if i == TokenTypes::FOR.literal() => {
                        push_token!(tokens, TokenTypes::FOR);
                    }
                    i if i == TokenTypes::WHILE.literal() => {
                        push_token!(tokens, TokenTypes::WHILE);
                    }
                    i if i == TokenTypes::FUNC.literal() => {
                        push_token!(tokens, TokenTypes::FUNC);
                    }
                    i if i == TokenTypes::WHEN.literal() => {
                        push_token!(tokens, TokenTypes::WHEN);
                    }
                    i if i == TokenTypes::USE.literal() => {
                        push_token!(tokens, TokenTypes::USE);
                    }
                    i if i == TokenTypes::OTHER.literal() => {
                        push_token!(tokens, TokenTypes::OTHER);
                    }
                    i if i == TokenTypes::IN.literal() => {
                        push_token!(tokens, TokenTypes::IN);
                    }

                    // Literals
                    i if i == TokenTypes::TRUE.literal() => {
                        push_token!(tokens, TokenTypes::TRUE);
                    }
                    i if i == TokenTypes::FALSE.literal() => {
                        push_token!(tokens, TokenTypes::FALSE);
                    }

                    // Comparison
                    i if i == TokenTypes::AND.literal() => {
                        push_token!(tokens, TokenTypes::AND);
                    }
                    i if i == TokenTypes::OR.literal() => {
                        push_token!(tokens, TokenTypes::OR);
                    }
                    _ => {
                        tokens.push(Token(TokenTypes::IDENT, identifier));
                    }
                }
            }
            c if c.is_ascii() && !c.is_alphanumeric() => {
                let mut identifier: String = String::new();
                while current_pos < input.len() && input_stream[current_pos].is_ascii() && !input_stream[current_pos].is_whitespace() && !input_stream[current_pos].is_alphanumeric() {
                    identifier.push(input_stream[current_pos]);
                    current_pos += 1;
                }
                match identifier {
                    i if i == TokenTypes::LCURLY.literal() => {
                        push_token!(tokens, TokenTypes::LCURLY);
                    }
                    i if i == TokenTypes::RCURLY.literal() => {
                        push_token!(tokens, TokenTypes::RCURLY);
                    }
                    i if i == TokenTypes::LPARENT.literal() => {
                        push_token!(tokens, TokenTypes::LPARENT);
                    }
                    i if i == TokenTypes::RPARENT.literal() => {
                        push_token!(tokens, TokenTypes::RPARENT);
                    }
                    i if i == TokenTypes::LSQUAREBRAC.literal() => {
                        push_token!(tokens, TokenTypes::LSQUAREBRAC);
                    }
                    i if i == TokenTypes::RSQUAREBRAC.literal() => {
                        push_token!(tokens, TokenTypes::RSQUAREBRAC);
                    }

                    // Arithmetic operations
                    i if i == TokenTypes::PLUS.literal() => {
                        push_token!(tokens, TokenTypes::PLUS);
                    }
                    i if i == TokenTypes::MINUS.literal() => {
                        push_token!(tokens, TokenTypes::MINUS);
                    }
                    i if i == TokenTypes::MULTIPLY.literal() => {
                        push_token!(tokens, TokenTypes::MULTIPLY);
                    }
                    i if i == TokenTypes::DIVIDE.literal() => {
                        push_token!(tokens, TokenTypes::DIVIDE);
                    }
                    i if i == TokenTypes::ASSIGN.literal() => {
                        push_token!(tokens, TokenTypes::ASSIGN);
                    }

                    // Comparison
                    i if i == TokenTypes::EQUAL.literal() => {
                        push_token!(tokens, TokenTypes::EQUAL);
                    }
                    i if i == TokenTypes::NOTEQUAL.literal() => {
                        push_token!(tokens, TokenTypes::NOTEQUAL);
                    }
                    i if i == TokenTypes::GREATERTHAN.literal() => {
                        push_token!(tokens, TokenTypes::GREATERTHAN);
                    }
                    i if i == TokenTypes::GREATEROREQUALTHAN.literal() => {
                        push_token!(tokens, TokenTypes::GREATEROREQUALTHAN);
                    }
                    i if i == TokenTypes::LESSERTHAN.literal() => {
                        push_token!(tokens, TokenTypes::LESSERTHAN);
                    }
                    i if i == TokenTypes::LESSEROREQUALTHAN.literal() => {
                        push_token!(tokens, TokenTypes::LESSEROREQUALTHAN);
                    }

                    // Misc types
                    i if i == TokenTypes::QUOTMARK.literal() => {
                        push_token!(tokens, TokenTypes::QUOTMARK);
                    }
                    i if i == TokenTypes::COMMENT.literal() => {
                        push_token!(tokens, TokenTypes::COMMENT);
                    }
                    i if i == TokenTypes::COMMA.literal() => {
                        push_token!(tokens, TokenTypes::COMMA);
                    }
                    i if i == TokenTypes::COLON.literal() => {
                        push_token!(tokens, TokenTypes::COLON);
                    }
                    i if i == TokenTypes::ARROW.literal() => {
                        push_token!(tokens, TokenTypes::ARROW);
                    }
                    _ => {}
                }
                current_pos += 1;
            }
            c if c.is_numeric() => {
                tokens.push(Token(TokenTypes::NUMBER, c.to_string()));
                current_pos += 1;
            }
            _ => {
                panic!("Not a valid character");
            }
        }
    }
    tokens
}
