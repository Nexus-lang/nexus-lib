#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Lexer, Literal, Operator, Token},
        util,
    };

    #[test]
    fn test_string() {
        let mut lexer = get_lexer("test");
        lexer.tokenize();
    }

    /// Test for checking if literals like numbers, booleans
    /// and strings get tokenized correctly
    #[test]
    fn test_tokenize_literals() {
        let mut lexer = get_lexer("literals");
        let expected = [
            // Strings
            // TODO: UTF-8 support
            Token::Literal(Literal::Str(String::from(
                "Hello, my name is John. I am a comedian entertaining cats",
            ))),
            // Integers
            Token::Literal(Literal::Num(9875986234.0)),
            // Integers with visual seperator
            Token::Literal(Literal::Num(1254_890.0)),
            // Floats
            Token::Literal(Literal::Num(5643877689.9886)),
            // Booleans
            Token::Literal(Literal::Bool(true)),
            Token::Literal(Literal::Bool(false)),
        ];
        for expect in expected {
            let tok = lexer.tokenize();
            lexer.tokenize();
            if let Some(tok) = tok {
                assert_eq!(expect, tok);
            }
        }
    }

    /// Test for checking if language-builtin keywords
    /// are tokenized correctly as well as checking idents
    /// that contain a keyword. This also server as the
    /// ident test
    #[test]
    fn test_keywords() {
        // TODO: UTF-8 support
        let mut lexer = get_lexer("keywords");
        let expected = [
            Token::Var,
            Token::Enum,
            Token::Else,
            Token::Const,
            Token::Loop,
            Token::Local,
            Token::Ident(String::from("vari")),
            Token::Ident(String::from("_const")),
            Token::Ident(String::from("iff")),
        ];
        for expect in expected {
            let tok = util::get_next_tok(&mut lexer);
            lexer.tokenize();
            assert_eq!(expect, tok)
        }
    }

    /// Test for checking if single character
    /// and multi character tokens are tokenized
    /// correctly
    #[test]
    fn test_symbols() {
        let mut lexer = get_lexer("symbols");
        let expected = [
            Token::Dot,
            Token::Comma,
            Token::Operator(Operator::Minus),
            Token::Eol,
            Token::Operator(Operator::Equals),
            Token::Operator(Operator::GreaterEquals),
            Token::Arrow,
        ];
        for expect in expected {
            let tok = util::get_next_tok(&mut lexer);
            lexer.tokenize();
            assert_eq!(expect, tok)
        }
    }

    /// Test for checking if both all-line
    /// and encased comments work
    #[test]
    fn test_comments() {
        let mut lexer = get_lexer("comments");
        let tok = util::get_next_tok(&mut lexer);
        assert_eq!(Token::Var, tok);
        let next_tok = util::get_next_tok(&mut lexer);
        assert_eq!(Token::Eof, next_tok);
    }

    fn get_lexer(test: &str) -> Lexer {
        Lexer::new(&format!("tests/lexer/{}.nx", test)).expect("Failed to open file")
    }
}
