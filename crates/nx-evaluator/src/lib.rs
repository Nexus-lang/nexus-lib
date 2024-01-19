use std::{cell::RefCell, rc::Rc};

use builtins::{BuiltinFunc, Input, Print};
use env::{EnvObj, Environment};
use nx_lexer::tokens::Literal;
use objects::{Comparable, FuncObj, Object};
use nx_parser::ast::{
    CallExpr, Expression, FuncExpr, Ident, InfixExpr, InfixOp, PrefixExpr, Statement, VarStmt, PrefixOp,
};

pub mod builtins;
pub mod env;
pub mod objects;
mod tests;
pub mod util;

pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn eval_stmt(&mut self, stmt: Statement) -> Object {
        match stmt {
            Statement::Variable(node) => self.eval_var(node),
            Statement::Return(_) => todo!(),
            Statement::Break(_) => todo!(),
            Statement::Local(_) => todo!(),
            Statement::Use(_) => todo!(),
            Statement::Expression(node) => self.eval_expr(node),
        }
    }

    fn eval_expr(&mut self, expr: Expression) -> Object {
        match expr {
            Expression::Ident(node) => self.eval_ident(node),
            Expression::Literal(node) => Object::Lit(node),
            Expression::Prefix(node) => self.eval_prefix(node),
            Expression::Infix(node) => self.eval_infix(node),
            Expression::Index(_) => todo!(),
            Expression::Call(node) => self.eval_call(node),
            Expression::List(_) => todo!(),
            Expression::None => todo!(),
            Expression::If(_) => todo!(),
            Expression::Loop(_) => todo!(),
            Expression::When(_) => todo!(),
            Expression::Func(node) => self.eval_func(node),
            Expression::Annotation(_) => todo!(),
            Expression::Struct(_) => todo!(),
            Expression::Enum(_) => todo!(),
        }
    }

    fn eval_var(&mut self, node: VarStmt) -> Object {
        let val = self.eval_expr(node.val);
        self.env.borrow_mut().set(
            node.name.ident.0.clone(),
            EnvObj {
                obj: val.clone(),
                is_const: node.is_const,
            },
        );
        val
    }

    fn eval_func(&mut self, node: FuncExpr) -> Object {
        Object::Func(FuncObj {
            args: node.args,
            block: node.block,
        })
    }

    fn eval_call(&mut self, node: CallExpr) -> Object {
        let name = match *node.ident {
            Expression::Ident(ident) => ident.0.clone(),
            _ => panic!("The function name is not an identifier"),
        };

        match name.as_str() {
            "print" => Object::BuiltinFunc(BuiltinFunc::Print(Print::print_val(Some(
                self.eval_expr(match node.args.get(0) {
                    Some(val) => val.clone(),
                    None => return Object::BuiltinFunc(BuiltinFunc::Print(Print::print_val(None))),
                }),
            )))),
            "input" => Object::BuiltinFunc(BuiltinFunc::Input(Input::input(None))),
            _ => {
                todo!()
            }
        }
    }

    fn eval_ident(&mut self, node: Ident) -> Object {
        match self.env.borrow_mut().get(node.0.clone()) {
            Some(obj) => obj.obj.clone(),
            None => panic!("Could not find identifier: {}", node.0),
        }
    }

    fn eval_prefix(&mut self, node: PrefixExpr) -> Object {
        match &node.op {
            PrefixOp::Pos => self.eval_expr(*node.val),
            PrefixOp::Neg => {
                let val = self.eval_expr(*node.val);
                Object::Lit(Literal::Num(match val {
                    Object::Lit(lit) => match lit {
                        Literal::Num(num) => -num,
                        _ => panic!("The obj is not a number"),
                    },
                    _ => panic!("The obj does not evaluate to a literal"),
                }))
            }
            PrefixOp::Not => {
                let val = self.eval_expr(*node.val);
                Object::Lit(Literal::Bool(match val {
                    Object::Lit(lit) => match lit {
                        Literal::Bool(bool) => !bool,
                        _ => panic!("The obj is not a boolean"),
                    },
                    _ => panic!("The obj does not evaluate to a literal"),
                }))
            }
        }
    }

    fn eval_infix(&mut self, node: InfixExpr) -> Object {
        self.eval_infix_from_num(node.op, *node.left, *node.right)
    }

    fn eval_infix_from_num(&mut self, op: InfixOp, left: Expression, right: Expression) -> Object {
        match op {
            InfixOp::Add => {
                let (left, right) = self.eval_infix_to_num(left, right);
                Object::Lit(Literal::Num(left + right))
            }
            InfixOp::Sub => {
                let (left, right) = self.eval_infix_to_num(left, right);
                Object::Lit(Literal::Num(left - right))
            }
            InfixOp::Mul => {
                let (left, right) = self.eval_infix_to_num(left, right);
                Object::Lit(Literal::Num(left * right))
            }
            InfixOp::Div => {
                let (left, right) = self.eval_infix_to_num(left, right);
                Object::Lit(Literal::Num(left / right))
            }
            InfixOp::Eq => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left == right))
            }
            InfixOp::NEq => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left != right))
            }
            InfixOp::GT => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left > right))
            }
            InfixOp::LT => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left < right))
            }
            InfixOp::GTEq => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left >= right))
            }
            InfixOp::LTEq => {
                let (left, right) = self.eval_infix_to_comp(left, right);
                Object::Lit(Literal::Bool(left <= right))
            }
            InfixOp::As => todo!(),
            InfixOp::In => todo!(),
            InfixOp::Range => todo!(),
            InfixOp::Assign => todo!(),
        }
    }

    fn eval_infix_to_num(&mut self, left: Expression, right: Expression) -> (f64, f64) {
        let left = self.eval_expr(left);
        let right = self.eval_expr(right);
        return (
            match Self::conv_to_num(left) {
                Some(val) => val,
                None => panic!("Left of the infix expression cannot is not a number"),
            },
            match Self::conv_to_num(right) {
                Some(val) => val,
                None => panic!("Right of the infix expression cannot is not a number"),
            },
        );
    }

    fn eval_infix_to_comp(
        &mut self,
        left: Expression,
        right: Expression,
    ) -> (Comparable, Comparable) {
        let left = self.eval_expr(left);
        let right = self.eval_expr(right);
        return (
            match left {
                Object::Lit(lit) => Comparable::Lit(lit),
                Object::None => Comparable::None,
                _ => panic!(
                    "Cannot compare left: {:#?} since it is not valid for comparison",
                    &left
                ),
            },
            match right {
                Object::Lit(lit) => Comparable::Lit(lit),
                Object::None => Comparable::None,
                _ => panic!(
                    "Cannot compare right: {:#?} since it is not valid for comparison",
                    &right
                ),
            },
        );
    }

    fn conv_to_num(obj: Object) -> Option<f64> {
        match obj {
            Object::Lit(lit) => match lit {
                Literal::Num(num) => Some(num),
                _ => None,
            },
            _ => None,
        }
    }
}
