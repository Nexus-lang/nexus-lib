use crate::tokens::{Token, TokenTypes};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()));
    };
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_pos = 0;
    let input_stream: Vec<char> = input.chars().collect();
    while current_pos < input.len() {
        let char = input_stream[current_pos];

        match char {
            c if c.is_whitespace() => {
                current_pos += 1;
            }
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

                    // Literals
                    i if i == TokenTypes::TRUE.literal() => {
                        push_token!(tokens, TokenTypes::TRUE);
                    }
                    i if i == TokenTypes::FALSE.literal() => {
                        push_token!(tokens, TokenTypes::FALSE);
                    }
                    _ => {
                        tokens.push(Token(TokenTypes::IDENT, identifier));
                    }
                }
            }
            c if c.is_ascii() && !c.is_alphanumeric() => {
                println!("{}", char);
                current_pos += 1;
            }
            c if c.is_numeric() => {
                println!("{}", char);
                current_pos += 1;
            }
            _ => {
                panic!("Not a valid character");
            }
        }
    }
    tokens
}
