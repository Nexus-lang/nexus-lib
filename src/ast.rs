use crate::tokens::TokenType;

pub trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Identifier {
    token: TokenType,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

/*
// Not needed atm
// Uncomment when used
impl Program {
    fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements: statements }
    }
}
*/

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}


struct VarStatement {
    token: TokenType, // var
    name: Identifier, // name
    value: Box<dyn Expression>,
}

impl Node for VarStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

impl Statement for VarStatement {
    fn statement_node(&self) {
        todo!()    
    }
}