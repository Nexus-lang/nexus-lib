use std::process::exit;

pub fn throw_error(message: &str) {
    #[allow(unused_variables)]
    let test = "e";
    println!("{}", message);
    exit(1);
}