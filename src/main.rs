mod tokens;
mod ast;
mod lexer;
mod parser;
mod util;
mod errors;
mod evaluator;
mod object;

use lexer::Lexer;
use parser::Parser;
use util::{FileHandler, input};

use crate::evaluator::Evaluator;

fn main() {
    // select the file to interpret
    println!("Enter file to interpret or type ENTER to use default file:");
    print!(">>");
    let input = input(); 

    let example_code = FileHandler::read_file(if input != "" {
        input.as_str()
    } else {
        "docs/examples/test.nx"
    });
    // ----

    // define lexer and parser
    let mut lexer = Lexer::new(example_code);

    let mut parser = Parser::new(&mut lexer);

    let evaluator = Evaluator::new(&mut parser);

    // tokenize input
    let token_stream = lexer.lex();

    // parse tokens
    let program = parser.parse_program();

    // evaluate program
    let debug_eval = evaluator.eval_program();

    // output values
    println!("TOKENS: \n");

    println!("{:?} \n", token_stream);

    println!("AST: \n");

    println!("{:?}", program);

    println!("DEBUG EVAL: \n");

    println!("{:?}", debug_eval);

    println!("PRESS ANY KEY TO EXIT");
    util::input();
}
