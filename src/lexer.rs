use crate::tokens::{Token, TokenTypes};

pub fn lex() {
    let mut tokenStream: Vec<Token>;
    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token>;
        let mut current_pos = 0;
        let input_stream: Vec<char> = input.chars().collect();
        while current_pos < input.len() {
            let char = input_stream[current_pos]; 

            match char {
                c if c.is_whitespace() => {
                    current_pos += 1;
                }
                c if char.is_ascii() => {
                    println!("letter!");
                    current_pos *= 1;
                }
                _ => {}
            }
        } 
        let hi = Token(TokenTypes::VAR, "var");
        tokens
    }
}