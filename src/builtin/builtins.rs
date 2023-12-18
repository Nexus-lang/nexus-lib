use clutils::literal::{LiteralStr, LiteralString};

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

impl LiteralStr for BuiltinType {
    fn literal(&self) -> &str {
        match self {
            BuiltinType::STRING => "String",
            BuiltinType::NUMBER => "Number",
            BuiltinType::BOOLEAN => "Boolean",
        }
    }
}

impl LiteralStr for BuiltinFunction {
    fn literal(&self) -> &str {
        match self {
            Self::PRINT => "print",
            Self::INPUT => "input",
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
        input()
    }
}
