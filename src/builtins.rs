use crate::{
    object::{self, Literal},
    util::input,
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

impl Literal for BuiltinType {
    fn literal(&self) -> String {
        match self {
            BuiltinType::STRING => String::from("Str"),
            BuiltinType::NUMBER => String::from("Num"),
            BuiltinType::BOOLEAN => String::from("Bool"),
        }
    }
}

impl Literal for BuiltinFunction {
    fn literal(&self) -> String {
        match self {
            Self::PRINT => String::from("print"),
            Self::INPUT => String::from("input"),
        }
    }
}

impl BuiltinFunction {
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
