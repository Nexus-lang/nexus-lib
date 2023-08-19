#[macro_use]
mod tokens;
mod ast;
mod lexer;
mod parser;
mod util;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let example_code = util::FileHandler::read_file("examples/interpreter_testing.nx");

    let mut lexer = Lexer::new(example_code);

    let token_stream = lexer.lex();

    let mut parser = Parser::new(&mut lexer);

    println!("{:?} \n", token_stream);

    /*
    println!("");
    println!("----------------------------------");

    let mut reconstructed: Vec<String> = vec!["".to_string()];
    for token in &token_stream {
        if token.token_type != TokenType::EOL && token.token_type != TokenType::EOF {
            reconstructed.push(token.literal.to_string());
        } else if token.literal == TokenType::EOL.literal() {
            reconstructed.push("\n".to_string())
        }
    }

    println!("{}", reconstructed.join(" "));

    println!("");
     */

    println!("AST: \n");

    println!("{:?}", parser.parse_program());
}

