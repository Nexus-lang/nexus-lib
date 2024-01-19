#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_parser() {
        let mut lexer = Lexer::new(&"tests/test.nx".into()).expect("Invalid file path");
        let mut parser = Parser::new(&mut lexer);
        let stmt = parser.parse_stmt().expect("Failed to parse");
        println!("{:#?}", stmt);
    }
}
