mod tokens;

use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let content = read_file("examples/main.nex");
    println!("{}", content);
}

fn read_file(path: &str) -> String {
    let char_path: Vec<char> = path.chars().collect();
    let path_length = char_path.len() - 1;
    let file_ending: Vec<char> = vec![char_path[path_length - 3], char_path[path_length - 2], char_path[path_length - 1], char_path[path_length]];
    let file_ending_string: String = file_ending.into_iter().collect();
    println!("{}", file_ending_string);
    let mut file = File::open(path).expect("Failed to find file");    
    let mut buffer = String::new();

    if file_ending_string != ".nex" {
        panic!("Not a valid nexus file");
    }

    file.read_to_string(&mut buffer)
        .expect("Failed to read file");

    buffer
}
