use crate::tokens::{Token, TokenType};

macro_rules! push_token {
    ($tokens:expr, $variant:path) => {
        $tokens.push(Token($variant, $variant.literal()))
    };
}

//TODO: the issue with characters in front of other characters in identifiers not being read needs to be fixed again
pub fn lex(input: String) -> Vec<Token> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_pos = 0;
    while current_pos < input_chars.len() {
        let ch = input_chars[current_pos];

        match ch {
            // new lines
            c if c == '\n' => {
                push_token!(tokens, TokenType::EOL);
                current_pos += 1;
            }
            // skipping white spaces
            c if c.is_whitespace() && c != '\n' => {
                current_pos += 1;
            }
            c if !c.is_whitespace() => {
                let mut identifier = String::new();
                identifier.push(ch);

                let mut next_pos = current_pos + 1;
                while next_pos < input_chars.len() && !input_chars[next_pos].is_whitespace() {
                    identifier.push(input_chars[next_pos]);
                    next_pos += 1;
                }

                current_pos = next_pos;

                match identifier {
                    i if i == TokenType::VAR.literal() => {
                        push_token!(tokens, TokenType::VAR);
                    }
                    i if i == TokenType::CONST.literal() => {
                        push_token!(tokens, TokenType::CONST)
                    }
                    i if i == TokenType::IF.literal() => {
                        push_token!(tokens, TokenType::IF);
                    }
                    i if i == TokenType::ELSE.literal() => {
                        push_token!(tokens, TokenType::ELSE);
                    }
                    i if i == TokenType::FOR.literal() => {
                        push_token!(tokens, TokenType::FOR);
                    }
                    i if i == TokenType::WHILE.literal() => {
                        push_token!(tokens, TokenType::WHILE);
                    }
                    i if i == TokenType::FUNC.literal() => {
                        push_token!(tokens, TokenType::FUNC);
                    }
                    i if i == TokenType::WHEN.literal() => {
                        push_token!(tokens, TokenType::WHEN);
                    }
                    i if i == TokenType::USE.literal() => {
                        push_token!(tokens, TokenType::USE);
                    }
                    i if i == TokenType::OTHER.literal() => {
                        push_token!(tokens, TokenType::OTHER);
                    }
                    i if i == TokenType::IN.literal() => {
                        push_token!(tokens, TokenType::IN);
                    }

                    // Literals
                    i if i == TokenType::TRUE.literal() => {
                        push_token!(tokens, TokenType::TRUE);
                    }
                    i if i == TokenType::FALSE.literal() => {
                        push_token!(tokens, TokenType::FALSE);
                    }

                    // Comparison
                    i if i == TokenType::AND.literal() => {
                        push_token!(tokens, TokenType::AND);
                    }
                    i if i == TokenType::OR.literal() => {
                        push_token!(tokens, TokenType::OR);
                    }

                    // Data Structures
                    i if i == TokenType::STRUCT.literal() => {
                        push_token!(tokens, TokenType::STRUCT);
                    }
                    i if i == TokenType::ENUM.literal() => {
                        push_token!(tokens, TokenType::ENUM);
                    }
                    i if i == TokenType::LCURLY.literal() => {
                        push_token!(tokens, TokenType::LCURLY);
                    }
                    i if i == TokenType::RCURLY.literal() => {
                        push_token!(tokens, TokenType::RCURLY);
                    }
                    i if i == TokenType::LPARENT.literal() => {
                        push_token!(tokens, TokenType::LPARENT);
                    }
                    i if i == TokenType::RPARENT.literal() => {
                        push_token!(tokens, TokenType::RPARENT);
                    }
                    i if i == TokenType::LSQUAREBRAC.literal() => {
                        push_token!(tokens, TokenType::LSQUAREBRAC);
                    }
                    i if i == TokenType::RSQUAREBRAC.literal() => {
                        push_token!(tokens, TokenType::RSQUAREBRAC);
                    }

                    // Arithmetic operations
                    i if i == TokenType::PLUS.literal() => {
                        push_token!(tokens, TokenType::PLUS);
                    }
                    i if i == TokenType::MINUS.literal() => {
                        push_token!(tokens, TokenType::MINUS);
                    }
                    i if i == TokenType::MULTIPLY.literal() => {
                        push_token!(tokens, TokenType::MULTIPLY);
                    }
                    i if i == TokenType::DIVIDE.literal() => {
                        push_token!(tokens, TokenType::DIVIDE);
                    }
                    i if i == TokenType::ASSIGN.literal() => {
                        push_token!(tokens, TokenType::ASSIGN);
                    }

                    // Comparison
                    i if i == TokenType::EQUAL.literal() => {
                        push_token!(tokens, TokenType::EQUAL);
                    }
                    i if i == TokenType::NOTEQUAL.literal() => {
                        push_token!(tokens, TokenType::NOTEQUAL);
                    }
                    i if i == TokenType::GREATERTHAN.literal() => {
                        push_token!(tokens, TokenType::GREATERTHAN);
                    }
                    i if i == TokenType::GREATEROREQUALTHAN.literal() => {
                        push_token!(tokens, TokenType::GREATEROREQUALTHAN);
                    }
                    i if i == TokenType::LESSERTHAN.literal() => {
                        push_token!(tokens, TokenType::LESSERTHAN);
                    }
                    i if i == TokenType::LESSEROREQUALTHAN.literal() => {
                        push_token!(tokens, TokenType::LESSEROREQUALTHAN);
                    }

                    // Misc types
                    i if i == TokenType::QUOTMARK.literal() => {
                        push_token!(tokens, TokenType::QUOTMARK);
                    }
                    i if i == TokenType::COMMENT.literal() => {
                        push_token!(tokens, TokenType::COMMENT);
                    }
                    i if i == TokenType::COMMA.literal() => {
                        push_token!(tokens, TokenType::COMMA);
                    }
                    i if i == TokenType::COLON.literal() => {
                        push_token!(tokens, TokenType::COLON);
                    }
                    i if i == TokenType::ARROW.literal() => {
                        push_token!(tokens, TokenType::ARROW);
                    }
                    i if i == TokenType::EXCLAMMARK.literal() => {
                        push_token!(tokens, TokenType::EXCLAMMARK)
                    }
                    _ => {
                        tokens.push(Token(TokenType::IDENT, identifier));
                    }
                }
            }
            _ => {
                println!("uwu");
                current_pos += 1;
            }
        }
    }

    tokens
}

/*
fn is_utf8_and_not_ascii(c: char) -> bool {
    let s = c.to_string();
    let bytes = s.as_bytes();

    // Check if the bytes form a valid UTF-8 string

    if !s.is_ascii() {
        if let Ok(_) = std::str::from_utf8(bytes) {
            return true;
        } else {
            return false;
        }
    }
    false
}
*/