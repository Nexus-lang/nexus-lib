use crate::{object::{self, Literal, Error}, util::{input, throw_error}};

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
        println!("{}", (func.args[0]).literal())
    }

    pub fn read_input(func: &object::BuiltInFunction) -> String {
        if func.args.len() == 1 {
            print!("{}", func.args[0].literal())
        } else if func.args.len() > 1 {
            throw_error(&Error::new("Function cannot have more than one argument"))
        }
        input()
    }
}