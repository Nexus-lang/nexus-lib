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
                println!("whitespace");
                current_pos += 1;
            }
            c if c.is_ascii() => {
                let mut identifier: Vec<char> = Vec::new();
                while current_pos < input.len() && input_stream[current_pos].is_ascii() && !input_stream[current_pos].is_whitespace() {
                    identifier.push(input_stream[current_pos]);
                    current_pos += 1;
                }
                let identifier_string: String = identifier.iter().collect();
                let identifier_str = identifier_string.as_str();
                println!("{}", identifier_str);

                match identifier_str {
                    i if i == TokenTypes::VAR.literal() => {
                        push_token!(tokens ,TokenTypes::VAR);
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
                    _ => {}
                }
            }
            _ => {}
        }
    }
    tokens
}
