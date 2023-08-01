use crate::tokens::{Token, TokenType};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()))
    };
}

//TODO: the issue with characters in front of other characters in identifiers not being read needs to be fixed again
pub fn lex(input: String) -> Vec<Token> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_pos = 0;
    while current_pos < input_chars.len() {
        let ch = input_chars[current_pos];

        match ch {
            c if c.is_alphabetic() || is_utf8_and_not_ascii(c)=> {
                let mut identifier = String::new();
                identifier.push(ch);

                let mut next_pos = current_pos + 1;
                while next_pos < input_chars.len() && (input_chars[next_pos].is_alphanumeric() || is_utf8_and_not_ascii(input_chars[next_pos])) {
                    identifier.push(input_chars[next_pos]);
                    next_pos += 1;
                }

                current_pos = next_pos;

                match identifier.as_str() {
                    i if i == TokenType::VAR.literal() => {
                        push_token!(tokens, TokenType::VAR)
                    }
                    i if i == TokenType::ASSIGN.literal() => {
                        push_token!(tokens, TokenType::ASSIGN)
                    }
                    _ => {
                        tokens.push(Token(TokenType::IDENT, identifier));
                    }
                }
            }
            _ => {
                println!("uwu");
                current_pos += 1;
            }
        }
    }
    tokens
}

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
