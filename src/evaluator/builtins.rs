use std::io;

use crate::parser::ast::Literal;

use super::objects::Object;

#[derive(Debug, Clone)]
pub enum BuiltinFunc {
    Print(Print),
    Input(Input),
}

impl BuiltinFunc {
    pub fn get_ret_val(&self) -> Option<Object> {
        match self {
            BuiltinFunc::Print(_) => None,
            BuiltinFunc::Input(input) => Some(Object::Lit(input.ret_val.clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    // Always a string literal
    // Use Literal instead of Object
    // to avoid using a Box
    ret_val: Literal,
}

impl Input {
    pub fn new(print_val: Option<String>) -> Self {
        let mut input = String::new();

        if let Some(val) = print_val {
            println!("{}", val)
        }

        // Read a line from the standard input and handle potential errors
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => eprintln!("Error reading input: {}", error),
        }
        Self {
            ret_val: Literal::Str(input),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Print;

impl Print {
    pub fn new(args: &Vec<Object>) -> Self {
        let mut buf = String::new();
        for arg in args {
            buf.push_str(&arg.to_string());
            buf.push(' ')
        }
        println!("{}", buf);

        Self
    }
}
