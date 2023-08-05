#[derive(PartialEq)]
pub enum Statement {
    VAR(VarStatement),
    EMPTY,
}

#[derive(PartialEq, PartialOrd)]
enum Expression {

}

#[derive(PartialEq, PartialOrd)]
struct Identifier {
    value: String
}

#[derive(PartialEq, PartialOrd)]
struct VarStatement {
    name: Identifier,
    value: Expression,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements: statements }
    }
}