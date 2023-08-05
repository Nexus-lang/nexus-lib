#[macro_use]
mod tokens;
mod ast;
mod errors;
mod lexer;
mod parser;

use errors::throw_error;
use std::fs::File;
use std::io::Read;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tokens::TokenType;

fn main() {
    let example_code = read_file("examples/test.nx");

    let mut lexer = Lexer::new(example_code);

    let token_stream = lexer.lex();

    let mut parser = Parser::new(&mut lexer);

    parser.next_token();
    
    println!("{:?}", token_stream);

    println!("");
    println!("----------------------------------");

    let mut reconstructed: Vec<String> = vec!["".to_string()];
    for token in &token_stream {
        if token.0 != TokenType::EOL && token.0 != TokenType::EOF {
            reconstructed.push(token.1.to_string());
        } else if token.1 == TokenType::EOL.literal() {
            reconstructed.push("\n".to_string())
        }
    }

    println!("{}", reconstructed.join(" "));

    println!("")
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to find file");
    let mut buffer = String::new();
    let file_ending = match path.split('.').last() {
        Some(ending) => ending,
        None => "unknown",
    };

    let error_message = format!("Wrong file format. Current: {}, expected: nex", file_ending);

    if file_ending != "nex" && file_ending != "nx" {
        throw_error(&error_message);
    }

    file.read_to_string(&mut buffer)
        .expect("Failed to read file");

    buffer
}
