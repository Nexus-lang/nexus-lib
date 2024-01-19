#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(&"tests/test.nx".into()).expect("Failed to open file");
        println!("{:#?}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
        println!("{}", lexer.tokenize());
    }
}