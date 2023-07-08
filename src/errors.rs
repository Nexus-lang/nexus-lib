use std::process::exit;

pub fn throw_error(message: &str) {
    println!("{}", message);
    exit(1);
}