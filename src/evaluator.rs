use crate::{
    ast::{Expression, Program, Statement},
    object::*,
    parser::Parser,
};

pub struct Evaluator {
    parser: Parser,
    program: Program,
}

impl Evaluator {
    pub fn new(parser: &mut Parser) -> Self {
        let program = parser.parse_program();
        Self { parser: parser.clone(), program }
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
            Expression::PREFIX(_) => todo!(),
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
}
