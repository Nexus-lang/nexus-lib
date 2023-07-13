use crate::tokens::{token_to_string, Token, TokenTypes};

pub fn lex() {
    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_pos = 0;
        let input_stream: Vec<char> = input.chars().collect();
        while current_pos < input.len() {
            let char = input_stream[current_pos];

            match char {
                c if c.is_whitespace() => {
                    current_pos += 1;
                }
                c if c.is_ascii() => {
                    tokens.push(Token(TokenTypes::ASSIGN, "assign"));
                    current_pos += 1;
                }
                _ => {}
            }
        }
        tokens
    }
    let token_stream = tokenize("uii");
    for token in token_stream {
        println!("{}", token.to_string())
    }
}
