use std::{cell::RefCell, rc::Rc};

use env::Environment;
use objects::Object;
use parser::ast::{Statement, VarStmt, Expression};

pub mod builtins;
pub mod env;
pub mod objects;
mod tests;

pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Statement) -> Object {
        match stmt {
            Statement::Variable(node) => self.eval_var(node),
            Statement::Return(_) => todo!(),
            Statement::Break(_) => todo!(),
            Statement::Local(_) => todo!(),
            Statement::Use(_) => todo!(),
            Statement::Expression(_) => todo!(),
        }
    }

    pub fn eval_expr(&mut self, expr: &Expression) -> Object {
        match expr {
            Expression::Ident(_) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::Prefix(_) => todo!(),
            Expression::Infix(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::List(_) => todo!(),
            Expression::None => todo!(),
            Expression::If(_) => todo!(),
            Expression::Loop(_) => todo!(),
            Expression::When(_) => todo!(),
            Expression::Func(_) => todo!(),
            Expression::Annotation(_) => todo!(),
            Expression::Struct(_) => todo!(),
            Expression::Enum(_) => todo!(),
        }
    }

    pub fn eval_var(&mut self, node: &VarStmt) -> Object {
        let val = self.eval_expr(&node.val);
        self.env
            .borrow_mut()
            .set(node.name.ident.0.clone(), val.clone());
        val
    }
}
