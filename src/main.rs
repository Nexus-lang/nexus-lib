#[macro_use]
mod tokens;
mod errors;
mod lexer;
mod parser;
mod ast;

use errors::throw_error;
use std::fs::File;
use std::io::Read;

use crate::lexer::lex;

fn main() {

    let example_code = &read_file("examples/test_2.nex");

    let token_stream = lex(example_code);
    for token in &token_stream {
        println!("{:?}", token);
    }

    println!("");
    println!("----------------------------------");

    let mut reconstructed: Vec<String> = vec![];
    for token in &token_stream {
        reconstructed.push(token.1.to_string());
    }

    println!("{}", reconstructed.join(" "));
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
