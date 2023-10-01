mod ast;
mod builtins;
mod enviroment;
mod errors;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod tokens;
mod util;

use std::{env, time::Instant};

use lexer::Lexer;
use parser::Parser;
use util::{input, FileHandler};

use crate::{evaluator::Evaluator, object::Error, util::throw_error};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "dbg" {
        // select the file to interpret
        println!("Enter file to interpret or press ENTER to use default file:");
        print!(">>");
        let input_file = input();
        let dbg_enabled = dbg_enabled();

        let start_time = Instant::now();

        let example_code = FileHandler::read_file(if input_file != "" {
            input_file.as_str()
        } else {
            "docs/examples/test.nx"
        });
        // ----

        // LEXER
        let mut lexer = Lexer::new(example_code);

        // tokenize input
        let token_stream = lexer.lex(None);

        // output values
        if dbg_enabled {
            println!("TOKENS: \n");

            println!("{:?} \n", token_stream);
        }

        // PARSER
        let mut parser = Parser::new(&mut lexer, dbg_enabled).unwrap();

        // parse tokens
        let program = parser.parse_program();

        if dbg_enabled {
            println!("AST: \n");

            println!("{:?}", &program);
        }

        println!("Output:");

        // EVALUATOR
        let mut evaluator = Evaluator::new(program);

        // evaluate program
        let debug_eval = evaluator.eval_program();

        println!();

        let elapsed_time = start_time.elapsed();

        if dbg_enabled {
            println!("DEBUG EVAL: \n");

            println!("{:?}", debug_eval);
        }

        let seconds = elapsed_time.as_secs();
        let millis = elapsed_time.subsec_millis();

        println!(
            "Execution time: {} seconds {} milliseconds",
            seconds, millis
        );

        print!("PRESS ANY KEY TO EXIT");
        util::input();
    } else {
        let source_file = FileHandler::read_file(args[1].as_str());
        let mut lexer = Lexer::new(source_file);
        let mut parser = Parser::new(&mut lexer, false).expect("Failed to create parser");
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program);
        evaluator.eval_program();
    }
}

fn dbg_enabled() -> bool {
    println!("Enable debug mode (y/n) press ENTER to use debug mode (default)");
    print!(">>");
    let input_dbg = input();
    println!();
    let dbg_enabled = if input_dbg == "y" || input_dbg == "" {
        true
    } else if input_dbg == "n" {
        false
    } else {
        throw_error(&Error::new(format!(
            "Invalid input. Expected y/n or empty. Got `{}` instead",
            input_dbg
        )));
        panic!()
    };
    dbg_enabled
}
