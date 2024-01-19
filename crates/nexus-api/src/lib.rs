use nx_evaluator::Evaluator;
use nx_lexer::Lexer;
use nx_parser::Parser;

pub fn execute_program(path: &str) {
    // TODO: Update clutils to allow string slices for paths in filehandler
    let mut lexer = Lexer::new(&path.into()).expect(&format!(
        "Failed to open file at {}. Maybe the file path is incorrect",
        path
    ));
    let mut parser = Parser::new(&mut lexer);
    let mut evalutator = Evaluator::new();
    loop {
        let stmt = match parser.parse_stmt() {
            Ok(stmt) => stmt,
            Err(_) => break,
        };
        evalutator.eval_stmt(stmt);
        parser.next_token();
    }
}
