use std::fs::File;
use std::io::{Read, self, Write};

/// Handles files and stores file path
/// for error messages
#[derive(Clone)]
pub struct FileHandler {
    pub file_path: String,
    pub file_content: String,
}

pub trait ToChar {
    /// Returns first character of a string as a char.
    /// Most useful when converting string with only
    /// one character to a char.
    /// 
    /// Will panic if string is empty
    fn to_char(&self) -> char;
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

        if file_ending != "nex" && file_ending != "nx" {
            panic!("Wrong file format. Current: {}, expected: nex", file_ending);
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