use std::{
    cell::{RefCell, RefMut},
    f32::consts::E,
    rc::Rc,
};

use crate::parser::ast::{
    BlockStmt, CallExpr, Expression, FuncExpr, Ident, InfixExpr, InfixOp, Literal, PrefixExpr,
    PrefixOp, Statement, VarStmt,
};
use builtins::{BuiltinFunc, Input, Print};
use env::{EnvObj, Environment};
use objects::{Comparable, FuncObj, Object};

pub mod builtins;
pub mod env;
pub mod objects;
mod tests;

#[derive(Debug)]
pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
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
            Expression::If(node) => self.eval_if(node),
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
            "print" => {
                Object::BuiltinFunc(BuiltinFunc::Print(Print::new(&self.eval_args(node.args))))
            }
            "input" => Object::BuiltinFunc(BuiltinFunc::Input(Input::new(None))),
            _ => {
                let old_env = Rc::clone(&self.env);

                let mut call_args = Vec::new();

                let call_arg_len = node.args.len();

                for arg in node.args {
                    call_args.push(self.eval_expr(arg));
                }

                // Get the function and add arguments to self.env
                let func = {
                    let mut env = self.env.borrow_mut();

                    let func_obj = env.get(&name).unwrap_or_else(|| {
                        panic!("Failed to find a function with the name {}", &name)
                    });

                    let func = Self::get_func(func_obj.obj.clone()).unwrap_or_else(|| {
                        panic!("Failed to find a function with the name {}", &name)
                    });

                    if func.args.len() != call_arg_len {
                        panic!("Amount of expected args: {}, does not match amount of provided args: {} for function: {}", func.args.len(), call_arg_len, name)
                    }

                    for (arg, call_arg) in func.args.clone().into_iter().zip(call_args) {
                        env.set(
                            arg.ident.0,
                            EnvObj::new(call_arg, false),
                        );
                    }

                    func
                };

                let last = self.eval_block(func.block);

                self.env = old_env;
                match last {
                    Some(obj) => obj,
                    None => Object::Void,
                }
            }
        }
    }

    fn eval_args(&mut self, args: Vec<Expression>) -> Vec<Object> {
        args.into_iter().map(|arg| self.eval_expr(arg)).collect()
    }

    fn eval_block(&mut self, block: BlockStmt) -> Option<Object> {
        let len = block.stmts.len();

        for (i, stmt) in block.stmts.into_iter().enumerate() {
            if i == len {
                return Some(self.eval_stmt(stmt));
            }
            self.eval_stmt(stmt);
        }
        None
    }

    fn eval_ident(&mut self, node: Ident) -> Object {
        match self.env.borrow().get(&node.0.clone()) {
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
        (
            match Self::conv_to_num(left) {
                Some(val) => val,
                None => panic!("Left of the infix expression cannot is not a number"),
            },
            match Self::conv_to_num(right) {
                Some(val) => val,
                None => panic!("Right of the infix expression cannot is not a number"),
            },
        )
    }

    fn eval_infix_to_comp(
        &mut self,
        left: Expression,
        right: Expression,
    ) -> (Comparable, Comparable) {
        let left = self.eval_expr(left);
        let right = self.eval_expr(right);
        (
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
        )
    }

    fn conv_to_num(obj: Object) -> Option<f64> {
        match obj {
            Object::Lit(Literal::Num(num)) => Some(num),
            _ => None,
        }
    }

    fn get_func(obj: Object) -> Option<FuncObj> {
        match obj {
            Object::Func(func) => Some(func),
            _ => None,
        }
    }
}
