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

    // LEXER
    let mut lexer = Lexer::new(example_code);

    // tokenize input
    let token_stream = lexer.lex(None);

    // output values
    println!("TOKENS: \n");

    println!("{:?} \n", token_stream);
    
    // PARSER
    let mut parser = Parser::new(&mut lexer).unwrap();

    // parse tokens
    let program = parser.parse_program();


    println!("AST: \n");

    println!("{:?}", &program);

    // EVALUATOR
    let mut evaluator = Evaluator::new(program);

    // evaluate program
    let debug_eval = evaluator.eval_program();

    println!("DEBUG EVAL: \n");

    println!("{:?}", debug_eval);

    println!("PRESS ANY KEY TO EXIT");
    util::input();
}
