pub enum Statement {
    VAR(VarStatement),
}

enum Expression {

}

struct Identifier {
    value: String
}

struct VarStatement {
    name: Identifier,
    value: Expression,
}

pub struct Program {
    statements: Vec<Statement>,
}