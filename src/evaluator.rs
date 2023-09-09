use crate::{
    ast::{Expression, Program, Statement, PrefixExpression, Operator, BooleanType},
    object::*,
};

pub struct Evaluator {
    program: Program,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn eval_program(&self) -> Object {
        let mut result = Object::None(NoneLit);
        for statement in &self.program.statements {
            result = self.eval_statement(statement);
        }
        result
    }

    fn eval_statement(&self, statement: &Statement) -> Object {
        match statement {
            Statement::VAR(_) => todo!(),
            Statement::CONST(_) => todo!(),
            Statement::RETURN(_) => todo!(),
            Statement::LOCAL(_) => todo!(),
            Statement::EXPRESSION(expr_stmt) => self.eval_expression(&expr_stmt.expression),
            Statement::EMPTY => todo!(),
        }
    }

    fn eval_expression(&self, expression: &Expression) -> Object {
        match expression {
            Expression::IDENTIFIER(_) => todo!(),
            Expression::NUMBERLITERAL(lit) => Object::Num(Num { value: lit.value }),
            Expression::STRINGLITERAL(_) => todo!(),
            Expression::PREFIX(lit) => self.eval_prefix_expression(lit),
            Expression::INFIX(_) => todo!(),
            Expression::BOOLEAN(lit) => Object::Bool(Bool { value: lit.bool_type.clone() }),
            Expression::IF(_) => todo!(),
            Expression::WHILE(_) => todo!(),
            Expression::FOR(_) => todo!(),
            Expression::FUNC(_) => todo!(),
            Expression::CALL(_) => todo!(),
            Expression::LIST(_) => todo!(),
            Expression::INDEX(_) => todo!(),
            Expression::ANNOTATION(_) => todo!(),
            Expression::NONE(_) => Object::None(NoneLit),
            Expression::EMPTY => panic!("Cannot evalutate EMPTY"),
        }
    }

    fn eval_prefix_expression(&self, node: &PrefixExpression) -> Object {
        let right = self.eval_expression(&node.right);
        // TODO: error checking

        match node.operator {
            Operator::BANG => self.eval_bang_expression(&right),
            Operator::PLUS => right,
            Operator::MINUS => self.eval_minus_expression(&right),
            _ => todo!(),
        }
    }

    fn eval_bang_expression(&self, right: &Object) -> Object {
        match right {
            Object::Bool(obj) => match obj.value {
                BooleanType::TRUE => Object::Bool(Bool { value: BooleanType::FALSE }),
                BooleanType::FALSE => Object::Bool(Bool { value: BooleanType::TRUE }),
            },
            Object::None(_) => todo!(),
            _ => todo!(),
        }
    }

    fn eval_minus_expression(&self, right: &Object) -> Object {
        match right {
            Object::Num(num) => Object::Num(Num { value: -num.value }),
            _ => todo!(),
        }
    }
}
