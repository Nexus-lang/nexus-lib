mod tests;
pub mod tokens;

use clutils::{errors::FileHandlerError, files::FileHandler};
use tokens::*;


pub struct Lexer {
    filehandler: FileHandler,
    cur_char: Option<char>,
    cur_pos: usize,
    next_pos: usize,
}

impl Lexer {
    pub fn new(path: &String) -> Result<Self, FileHandlerError> {
        let filehandler = FileHandler::new(path)?;
        let mut lexer = Self {
            filehandler,
            cur_char: None,
            cur_pos: 0,
            next_pos: 0,
        };
        lexer.next_char();
        Ok(lexer)
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        self.skip_whitespace();
        Some(match self.cur_char {
            Some(ch) => match ch {
                '\n' => {
                    self.next_char();
                    Token::Eol
                }
                c if c.is_numeric() => self.tokenize_num(),
                c if c.is_alphabetic() || c == '_' => self.tokenize_ident(),
                _ => return self.tokenize_symbol(),
            },
            None => Token::Eof,
        })
    }

    fn tokenize_num(&mut self) -> Token {
        // TODO: floating points
        let first_pos = self.cur_pos;
        let mut found_fp = false;
        while let Some(ch) = self.cur_char {
            if ch.is_numeric() || ch == '_' {
                self.next_char();
            } else if !found_fp && ch == '.' {
                found_fp = true;
                self.next_char();
            } else {
                break;
            }
        }
        let string: String = self.filehandler.content[first_pos..self.cur_pos].into();
        // Remove all underscores to ensure that parsing works
        let string: String = string.chars().filter(|&c| c != '_').collect();
        Token::Literal(Literal::Num(
            string
                .parse()
                .expect(&format!("Failed to parse string: {} to an integer", string)),
        ))
    }

    fn tokenize_symbol(&mut self) -> Option<Token> {
        let ret = match self.cur_char {
            Some(ch) => match ch {
                '=' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Operator(Operator::Equals)
                    }
                    Some('>') => {
                        self.next_char();
                        Token::Arrow
                    }
                    _ => Token::Assign,
                },
                '+' => Token::Operator(Operator::Plus),
                '-' => Token::Operator(Operator::Minus),
                '!' => Token::ExclamMark,
                '*' => Token::Operator(Operator::Asterisk),
                '/' => Token::Operator(Operator::Slash),
                '>' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Operator(Operator::GreaterEquals)
                    }
                    _ => Token::Operator(Operator::Greater),
                },
                '<' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Operator(Operator::LesserEquals)
                    }
                    _ => Token::Operator(Operator::Lesser),
                },
                ';' => Token::Eol,
                '(' => Token::LParent,
                ')' => Token::RParent,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                '"' => self.tokenize_string(),
                ':' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some(':') => {
                        self.next_char();
                        Token::ConstAssign
                    }
                    Some('=') => {
                        self.next_char();
                        Token::VarAssign
                    }
                    _ => Token::Colon,
                },
                ',' => Token::Comma,
                '.' => Token::Dot,
                '#' => return self.tokenize_comment(),
                _ => panic!("Invalid symbol: {:?}", &self.cur_char),
            },
            None => todo!(),
        };
        self.next_char();
        Some(ret)
    }

    fn tokenize_string(&mut self) -> Token {
        self.next_char();
        let begin_pos = self.cur_pos;
        while self.filehandler.content.chars().nth(self.cur_pos) != Some('"') {
            if self.filehandler.content.chars().nth(self.cur_pos) == Some('{') {
                self.next_char();
                while self.filehandler.content.chars().nth(self.cur_pos) != Some('}') {
                    self.next_char();
                }
            }
            self.next_char();
        }
        let string = &self.filehandler.content[begin_pos..self.cur_pos];
        Token::Literal(Literal::Str(string.into()))
    }

    fn tokenize_comment(&mut self) -> Option<Token> {
        self.next_char();
        while let Some(cur_ch) = self.cur_char {
            if cur_ch != '\n' && cur_ch != '#' {
                self.next_char();
            } else {
                break;
            }
        }
        self.next_char();
        None
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.next_char();
            } else {
                break;
            }
        }
        let ident: String = self.filehandler.content[first_pos..self.cur_pos].into();
        return match ident.as_str() {
            "var" => Token::Var,
            "const" => Token::Const,
            "func" => Token::Func,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "use" => Token::Use,

            "loop" => Token::Loop,
            "if" => Token::If,
            "else" => Token::Else,
            "when" => Token::When,

            "and" => Token::And,
            "or" => Token::Or,

            "return" => Token::Return,
            "break" => Token::Break,
            "local" => Token::Local,

            "true" => Token::Literal(Literal::Bool(true)),
            "false" => Token::Literal(Literal::Bool(false)),

            _ => Token::Ident(self.filehandler.content[first_pos..self.cur_pos].into()),
        };
    }

    fn next_char(&mut self) {
        self.cur_pos = self.next_pos;
        self.cur_char = self.filehandler.content.chars().nth(self.cur_pos);
        self.next_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        match self.cur_char {
            Some(mut ch) => {
                while ch.is_whitespace() && ch != '\n' {
                    self.next_char();
                    match self.cur_char {
                        Some(cch) => ch = cch,
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}
