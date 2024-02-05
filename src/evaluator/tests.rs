#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    use crate::evaluator::Evaluator;

    #[test]
    fn test_evaluator() {
        let mut lexer = Lexer::new(&"tests/evaluator/test.nx".into()).expect("Failed to find file");
        let mut parser = Parser::new(&mut lexer);
        let mut evaluator = Evaluator::new();
        evaluator.eval_stmt(parser.parse_stmt().expect("Encountered eol"));
        parser.next_token();
        evaluator.eval_stmt(parser.parse_stmt().expect("Encountered eol"));
    }
}