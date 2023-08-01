#[derive(Debug)]
enum TokenType {
    HELLO,
    WORLD,
    IDENT,
}

impl TokenType {
    fn literal(&self) -> &'static str {
        match self {
            Self::HELLO => "Hello",
            Self::WORLD => "World",
            Self::IDENT => "IDENT",
        }
    }
}

#[derive(Debug)]
struct Token(pub TokenType, pub String);

fn main() {
    let test: Vec<char> = "Hello World 🦀".chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    println!("{:?}", test);

    let mut current_pos = 0;
    while current_pos < test.len() {
        let ch = test[current_pos];

        if !ch.is_whitespace() {
            let mut identifier = String::new();
            identifier.push(ch);

            let mut next_pos = current_pos + 1;
            while next_pos < test.len() && !test[next_pos].is_whitespace() {
                identifier.push(test[next_pos]);
                next_pos += 1;
            }

            current_pos = next_pos;

            match identifier.as_str() {
                i if i == TokenType::HELLO.literal() => {
                    tokens.push(Token(TokenType::HELLO, TokenType::HELLO.literal().to_string()))
                }
                i if i == TokenType::WORLD.literal() => {
                    tokens.push(Token(TokenType::WORLD, TokenType::WORLD.literal().to_string()))
                }
                _ => {
                    tokens.push(Token(TokenType::IDENT, identifier));
                }
            }
        } else {
            println!("uwu");
            current_pos += 1;
        }
    }

    println!("{:?}", tokens);
    println!("\"")
}
