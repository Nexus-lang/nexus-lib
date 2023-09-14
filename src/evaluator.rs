use crate::{
    ast::*,
    object::*,
};

pub struct Evaluator {
    program: Program,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    fn eval(&mut self, statement: &Statement) -> Object {
        self.eval_statement(statement)
    }

    pub fn eval_program(&mut self) -> Object {
        let mut result = Object::None(NoneLit);
        for statement in &self.program.statements.clone() {
            result = self.eval(statement);
            println!("program evalutation: {:?}", result);
        }
        result
    }

    fn eval_statement(&mut self, statement: &Statement) -> Object {
        match statement {
            Statement::VAR(_) => todo!(),
            Statement::CONST(_) => todo!(),
            Statement::RETURN(_) => todo!(),
            Statement::LOCAL(_) => todo!(),
            Statement::EXPRESSION(expr_stmt) => self.eval_expression(&expr_stmt.expression),
            Statement::EMPTY => todo!(),
            Statement::BLOCK(block) => self.eval_block_statement(block),
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Object {
        match expression {
            Expression::IDENTIFIER(_) => todo!(),
            Expression::NUMBERLITERAL(lit) => Object::Num(Num { value: lit.value }),
            Expression::STRINGLITERAL(_) => todo!(),
            Expression::PREFIX(lit) => self.eval_prefix_expression(lit),
            Expression::INFIX(lit) => self.eval_infix_expression(lit),
            Expression::BOOLEAN(lit) => Object::Bool(Bool { value: lit.bool_type.clone() }),
            Expression::IF(lit) => self.eval_if_expression(lit),
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

    fn eval_prefix_expression(&mut self, node: &PrefixExpression) -> Object {
        let right = self.eval_expression(&node.right);
        // TODO: error checking

        match node.operator {
            Operator::BANG => self.eval_bang_expression(right),
            Operator::PLUS => right,
            Operator::MINUS => self.eval_minus_expression(right),
            _ => todo!(),
        }
    }

    fn eval_infix_expression(&mut self, node: &InfixExpression) -> Object {
        let left = self.eval_expression(&node.left);
        let right = self.eval_expression(&node.right);
        let operator = &node.operator;

        if left.get_type() == ObjectType::NUMBER && right.get_type() == ObjectType::NUMBER {
            self.eval_integer_infix_expression(operator, left, right)
        } else if operator == &Operator::EQUAL {
            self.native_bool_to_object(left == right)
        } else if operator == &Operator::NOTEQUAL {
            self.native_bool_to_object(left != right)
        } else {
            todo!("support for all expressions")
        }
    }

    fn eval_integer_infix_expression(&mut self, operator: &Operator, left: Object, right: Object) -> Object {
        let left_val: f64;
        let right_val: f64;
        if let Object::Num(num) = left {
            left_val = num.value;
        } else {
            panic!("left value is not an integer")
        }

        if let Object::Num(num) = right {
            right_val = num.value;
        } else {
            panic!("right value is not an integer")
        }

        match operator {
            Operator::PLUS => Object::Num(Num { value: left_val + right_val }),
            Operator::MINUS => Object::Num(Num { value: left_val - right_val }),
            Operator::MULTIPLY => Object::Num(Num { value: left_val * right_val }),
            Operator::DIVIDE => Object::Num(Num { value: left_val / right_val }),
            Operator::GREATTHAN => self.native_bool_to_object(left_val > right_val),
            Operator::LESSTHAN => self.native_bool_to_object(left_val < right_val),
            Operator::GREATOREQUAL => self.native_bool_to_object(left_val >= right_val),
            Operator::LESSOREQUAL => self.native_bool_to_object(left_val <= right_val),
            Operator::EQUAL => self.native_bool_to_object(left_val == right_val),
            Operator::NOTEQUAL => self.native_bool_to_object(left_val != right_val),
            _ => Object::None(NoneLit)
        }
    }

    fn eval_block_statement(&mut self, block: &BlockStatement) -> Object {
        let mut result = Object::None(NoneLit);

        for stmt in block.statements.iter() {
            result = self.eval_statement(stmt);
        }

        result
    }

    fn eval_if_expression(&mut self, node: &IfExpression) -> Object {
        let condition = self.eval_expression(&node.condition.as_ref().clone());

        if self.is_truthy(condition) {
            println!("alternative: {:?}", node.alternative);
            return self.eval_block_statement(&node.consequence);
        } else if node.alternative != None {
            println!("evaluating alt");
            return self.eval_else_expression(&node.alternative.as_ref().unwrap());
        } else {
            todo!()
        }
    }

    fn eval_else_expression(&mut self, alternative: &Box<IfExpression>) -> Object {
        let alt = *alternative.clone();
        let condition = self.eval_expression(&Expression::IF(alt));

        if self.is_truthy(condition) {
            println!("sus if");
            return self.eval_block_statement(&alternative.consequence);
        } else if alternative.alternative != None {
            println!("sus else if");
            return self.eval_else_expression(&alternative.alternative.as_ref().unwrap());
        } else {
            println!("nothingness");
            Object::None(NoneLit)
        }
    }

    fn is_truthy(&mut self, object: Object) -> bool {
        match object {
            Object::Bool(bool) => match bool.value {
                BooleanType::TRUE => true,
                BooleanType::FALSE => false,
            },
            Object::None(_) => false,
            _ => todo!(),
        }
    }

    fn native_bool_to_object(&self, bool: bool) -> Object {
        match bool {
            true => Object::Bool(Bool { value: BooleanType::TRUE }),
            false => Object::Bool(Bool { value: BooleanType::FALSE }),
        }
    }

    fn eval_bang_expression(&self, right: Object) -> Object {
        match right {
            Object::Bool(obj) => match obj.value {
                BooleanType::TRUE => Object::Bool(Bool { value: BooleanType::FALSE }),
                BooleanType::FALSE => Object::Bool(Bool { value: BooleanType::TRUE }),
            },
            Object::None(_) => right,
            _ => todo!(),
        }
    }

    fn eval_minus_expression(&self, right: Object) -> Object {
        match right {
            Object::Num(num) => Object::Num(Num { value: -num.value }),
            _ => right,
        }
    }
}
