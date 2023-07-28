pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements: statements }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal() // Corrected call to amogus()
            // You can also use: Statement::amogus(); if it's a static method
        } else {
            "".to_string()
        } // Just for demonstration purposes; you can return the actual token literal here
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for stmt in &self.statements {
            result.push_str(&stmt.to_string());
        }
        result
    }
}
