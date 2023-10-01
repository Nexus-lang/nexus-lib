use std::fs::File;
use std::io::{Read, self, Write};
use std::process;

use colored::Colorize;

use crate::object::Error;

/// Handles files and stores file path
/// for error messages
#[derive(Clone)]
pub struct FileHandler {
    pub file_path: String,
    pub file_content: String,
}

pub trait FirstAsChar {
    /// Returns first character of a string as a char.
    /// Most useful when converting string with only
    /// one character to a char.
    /// 
    /// Will panic if string is empty
    fn first_as_char(&self) -> char;
}

pub trait ToSome<T> {
    /// Wraps a value in Some(...)
    /// Built for qol purposes :D
    fn to_some(self) -> Option<T>;
}

impl<T> ToSome<T> for T {
    fn to_some(self) -> Option<T> {
        Some(self)
    }
}

impl FileHandler {
    /// Constructs FileHandler from file path
    pub fn read_file(path: &str) -> FileHandler {
        let mut file = File::open(path).expect(format!("Failed to find file: {}", path).as_str());
        let mut buffer = String::new();
        let file_ending = match path.split('.').last() {
            Some(ending) => ending,
            None => "unknown",
        };

        if file_ending != "nx" {
            panic!("Wrong file format. Current: {}, expected: nx", file_ending);
        }

        file.read_to_string(&mut buffer)
            .expect("Failed to read file");

        FileHandler { file_path: path.to_string(), file_content: buffer }
    }
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
