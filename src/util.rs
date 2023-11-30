use std::fs::File;
use std::io::{Read, self, Write};
use std::process;

use colored::Colorize;

use crate::evaluator::object::Error;

pub trait FirstAsChar {
    /// Returns first character of a string as a char.
    /// Most useful when converting string with only
    /// one character to a char.
    /// 
    /// Will panic if string is empty
    fn first_as_char(&self) -> char;
}

/// Accept input from the console
pub fn input() -> String {
    io::stdout().flush().expect("Failed to flush output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()

}

pub fn throw_error(err: &Error) {
    println!("{} {}", "Error:".red(), err.message);
    process::exit(0);
}

/// Converts use path to actual file path
pub fn convert_path(use_path: &str) -> String {
    use_path.replace(".", "/")
}
