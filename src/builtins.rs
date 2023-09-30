use crate::{
    object::{self, Error, Literal},
    util::{input, throw_error},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BuiltinFunction {
    PRINT,
    INPUT,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BuiltinType {
    STRING,
    NUMBER,
    BOOLEAN,
}

impl BuiltinFunction {
    pub fn name(&self) -> String {
        match self {
            Self::PRINT => String::from("print"),
            Self::INPUT => String::from("input"),
        }
    }

    /* calling the functions */
    pub fn print_val(func: &object::BuiltInFunction) {
        println!();
        for (index, arg) in func.args.iter().enumerate() {
            if index + 1 < func.args.len() {
                print!("{} ", arg.literal())
            } else {
                print!("{}", arg.literal())
            }
        }
    }

    pub fn read_input(func: &object::BuiltInFunction) -> String {
        Self::print_val(func);
        input()
    }
}
