use crate::tokens::{Token, TokenType};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()))
    };
}

pub fn lex(input: String) -> Vec<Token> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_pos = 0;
    while current_pos < input_chars.len() {
        let ch = input_chars[current_pos];

        if !ch.is_whitespace() {
            let mut identifier = String::new();
            identifier.push(ch);

            let mut next_pos = current_pos + 1;
            while next_pos < input_chars.len() && !input_chars[next_pos].is_whitespace() {
                identifier.push(input_chars[next_pos]);
                next_pos += 1;
            }

            current_pos = next_pos;

            match identifier.as_str() {
                i if i == TokenType::VAR.literal() => {
                    tokens.push(Token(TokenType::VAR, TokenType::VAR.literal().to_string()))
                }
                i if i == TokenType::ASSIGN.literal() => {
                    tokens.push(Token(TokenType::ASSIGN, TokenType::ASSIGN.literal().to_string()))
                }
                _ => {
                    tokens.push(Token(TokenType::IDENT, identifier));
                }
            }
        } else {
            println!("uwu");
            current_pos += 1;
        }
    }
    tokens
}
