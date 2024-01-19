#[cfg(test)]
mod tests {
    use nx_lexer::Lexer;
    use nx_parser::Parser;

    use crate::Evaluator;

    #[test]
    fn test_evaluator() {
        let mut lexer = Lexer::new(&"tests/test.nx".into()).expect("Failed to find file");
        let mut parser = Parser::new(&mut lexer);
        let mut evaluator = Evaluator::new();
        evaluator.eval_stmt(parser.parse_stmt().expect("Encountered eol"));
        parser.next_token();
        evaluator.eval_stmt(parser.parse_stmt().expect("Encountered eol"));
    }
}