use crate::object::{self, Literal};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BuiltinFunction {
    PRINT
}

impl BuiltinFunction {
    pub fn name(&self) -> String {
        match self {
            Self::PRINT => String::from("print"),
        }
    }

    /* calling the functions */
    pub fn print_val(func: &object::BuiltInFunction) {
        println!("{}", (func.args[0]).literal())
    }
}