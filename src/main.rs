mod builtin;
mod evaluator;
mod lexer;
mod parser;
mod util;

use std::{env, time::Instant};

use builtin::errors;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use util::{input, FileHandler};

use crate::{
    evaluator::{evaluator::Evaluator, object::Error},
    util::throw_error,
};

// nexus run <File>

// nexus bundle Vec<Files>

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = match args.get(1) {
        Some(arg) => arg,
        None => todo!(),
    };

    let second_arg = match args.get(2) {
        Some(arg) => arg,
        None => todo!(),
    };

    match first_arg.as_str() {
        "run" => {
            let src = FileHandler::read_file(&second_arg);
            run_interpreter(src)
        },
        _ => todo!(),
    };
}

fn run_interpreter(src: FileHandler) {
    let mut lexer = Lexer::new(src);

    let mut parser = match Parser::new(&mut lexer, false) {
        Ok(parser) => parser,
        Err(_) => todo!(),
    };
    let ast = parser.parse_program();

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(ast);
}
