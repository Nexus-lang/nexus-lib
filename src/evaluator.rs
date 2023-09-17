use crate::{ast::*, object::*};

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
            result = match result {
                Object::Return(lit) => {return *lit.value.clone();},
                Object::Error(_) => return result,
                _ => result.clone(),
            };
        }
        result
    }

    fn eval_statement(&mut self, statement: &Statement) -> Object {
        match statement {
            Statement::VAR(_) => todo!(),
            Statement::CONST(_) => todo!(),
            Statement::RETURN(ret_stmt) => self.eval_return_statement(&ret_stmt),
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
            Expression::STRINGLITERAL(_) => Object::None(NoneLit),
            Expression::PREFIX(lit) => self.eval_prefix_expression(lit),
            Expression::INFIX(lit) => self.eval_infix_expression(lit),
            Expression::BOOLEAN(lit) => Object::Bool(Bool {
                value: lit.bool_type.clone(),
            }),
            Expression::IF(lit) => self.eval_if_expression(lit),
            Expression::WHILE(_) => todo!(),
            Expression::FOR(_) => todo!(),
            Expression::FUNC(_) => todo!(),
            Expression::CALL(_) => todo!(),
            Expression::LIST(_) => todo!(),
            Expression::INDEX(_) => todo!(),
            Expression::ANNOTATION(_) => todo!(),
            Expression::NONE(_) => Object::None(NoneLit),
            Expression::EMPTY => Object::Error(Error::new("Cannot evaluate EMPTY expression")),
        }
    }

    fn eval_prefix_expression(&mut self, node: &PrefixExpression) -> Object {
        let right = self.eval_expression(&node.right);
        // TODO: error checking

        match node.operator {
            Operator::BANG => self.eval_bang_expression(right),
            Operator::PLUS => right,
            Operator::MINUS => self.eval_minus_expression(right),
            _ => Object::Error(Error::new(format!("Illegal prefix operation: {:?}", node.operator).as_str())),
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
            Object::Error(Error::new(format!("Unknown operation: left: {:?}, right: {:?}, operator: {:?}", left, right, operator).as_str()))
        }
    }

    fn eval_integer_infix_expression(
        &mut self,
        operator: &Operator,
        left: Object,
        right: Object,
    ) -> Object {
        let left_val: f64;
        let right_val: f64;
        if let Object::Num(num) = left {
            left_val = num.value;
        } else {
            return Object::Error(Error::new(format!("left value is not a number. Expected number found: {:?} instead", left).as_str()));
        }

        if let Object::Num(num) = right {
            right_val = num.value;
        } else {
            return Object::Error(Error::new(format!("right value is not a number. Expected number found: {:?} instead", right).as_str()));
        }

        match operator {
            Operator::PLUS => Object::Num(Num {
                value: left_val + right_val,
            }),
            Operator::MINUS => Object::Num(Num {
                value: left_val - right_val,
            }),
            Operator::MULTIPLY => Object::Num(Num {
                value: left_val * right_val,
            }),
            Operator::DIVIDE => Object::Num(Num {
                value: left_val / right_val,
            }),
            Operator::GREATTHAN => self.native_bool_to_object(left_val > right_val),
            Operator::LESSTHAN => self.native_bool_to_object(left_val < right_val),
            Operator::GREATOREQUAL => self.native_bool_to_object(left_val >= right_val),
            Operator::LESSOREQUAL => self.native_bool_to_object(left_val <= right_val),
            Operator::EQUAL => self.native_bool_to_object(left_val == right_val),
            Operator::NOTEQUAL => self.native_bool_to_object(left_val != right_val),
            _ => Object::None(NoneLit),
        }
    }

    fn eval_block_statement(&mut self, block: &BlockStatement) -> Object {
        let mut result = Object::None(NoneLit);

        for stmt in block.statements.iter() {
            result = self.eval_statement(stmt);

            match result {
                Object::Return(_) => return result,
                _ => continue,
            }
        }

        result
    }

    fn eval_if_expression(&mut self, node: &IfExpression) -> Object {
        // sussy unweap
        let condition = match &node.condition.clone() {
            Some(condition) => self.eval_expression(&condition),
            None => Object::None(NoneLit),
        }; // &node.condition.as_ref().clone().unwrap()

        if condition != Object::None(NoneLit) && self.is_truthy(condition) {
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
        let condition = match &alt.condition.clone() {
            Some(cond) => self.eval_expression(cond),
            None => Object::None(NoneLit),
        };

        if condition != Object::None(NoneLit) && self.is_truthy(condition) {
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

    fn eval_return_statement(&mut self, ret_stmt: &ReturnStatement) -> Object {
        let value = Box::from(self.eval(&&Statement::EXPRESSION(ExpressionStatement {
            expression: ret_stmt.return_value.clone(),
        })));
        Object::Return(Return { value })
    }

    fn is_truthy(&mut self, object: Object) -> bool {
        match object {
            Object::Bool(bool) => match bool.value {
                BooleanType::TRUE => true,
                BooleanType::FALSE => false,
            },
            Object::None(_) => false,
            _ => todo!("{:?}", object),
        }
    }

    fn native_bool_to_object(&self, bool: bool) -> Object {
        match bool {
            true => Object::Bool(Bool {
                value: BooleanType::TRUE,
            }),
            false => Object::Bool(Bool {
                value: BooleanType::FALSE,
            }),
        }
    }

    fn eval_bang_expression(&self, right: Object) -> Object {
        match right {
            Object::Bool(obj) => match obj.value {
                BooleanType::TRUE => Object::Bool(Bool {
                    value: BooleanType::FALSE,
                }),
                BooleanType::FALSE => Object::Bool(Bool {
                    value: BooleanType::TRUE,
                }),
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
