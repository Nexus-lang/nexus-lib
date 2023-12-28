#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("test/main.nx".into());
        dbg!("{}", lexer.tokenize());
        dbg!("{}", lexer.tokenize());
        dbg!("{}", lexer.tokenize());
        dbg!("{}", lexer.tokenize());
    }
}