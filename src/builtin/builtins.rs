use crate::{evaluator::object, util::*};

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
            BuiltinType::STRING => String::from("String"),
            BuiltinType::NUMBER => String::from("Number"),
            BuiltinType::BOOLEAN => String::from("Boolean"),
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
        for (index, arg) in func.args.iter().enumerate() {
            if index + 1 < func.args.len() {
                print!("{} ", arg.literal())
            } else {
                print!("{}", arg.literal())
            }
        }
        println!();
    }

    pub fn read_input(func: &object::BuiltInFunction) -> String {
        Self::print_val(func);
        input()
    }
}
