use clutils::{file_handler::{Extension, FileHandler}, literal::Literal};
use evaluator::evaluator::Evaluator;
use lexer::lexer::Lexer;
use parser::parser::Parser;

mod builtin;
mod evaluator;
mod lexer;
mod parser;
mod util;

pub enum NexusExtensions {
    NX,
}

impl Literal for NexusExtensions {
    fn literal(self: &Self) -> &str {
        "nx"
    }
}

impl Extension for NexusExtensions {
    
}

pub fn run_interpreter(src: FileHandler) {
    let mut lexer = Lexer::new(src);

    let mut parser = match Parser::new(&mut lexer, true) {
        Ok(parser) => parser,
        Err(_) => todo!(),
    };

    let ast = parser.parse_program();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(ast);
}