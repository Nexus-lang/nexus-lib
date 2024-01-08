pub mod tokens;
mod tests;


use clutils::{errors::FileHandlerError, files::FileHandler};
use tokens::{Literal, Operator};

use self::tokens::Token;

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
            filehandler: filehandler,
            cur_char: None,
            cur_pos: 0,
            next_pos: 0,
        };
        lexer.next_char();
        Ok(lexer)
    }

    pub fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.cur_char {
            Some(ch) => match ch {
                c if c.is_numeric() => self.tokenize_num(),
                c if c.is_alphabetic() => self.tokenize_ident(),
                _ => self.tokenize_symbol(),
            },
            None => Token::Eof,
        };
        tok
    }

    fn tokenize_num(&mut self) -> Token {
        // TODO: floating points
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_numeric() {
            self.next_char();
        }
        let string: String = self.filehandler.content[first_pos..self.cur_pos].into();
        Token::Literal(Literal::Num(string.parse().unwrap()))
    }

    fn tokenize_symbol(&mut self) -> Token {
        let ret = match self.cur_char {
            Some(ch) => match ch {
                '=' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Operator(Operator::Equals)
                    }
                    _ => Token::Assign,
                },
                '+' => Token::Operator(Operator::Plus),
                '-' => Token::Operator(Operator::Minus),
                '*' => Token::Operator(Operator::Asterisk),
                '/' => Token::Operator(Operator::Slash),
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
                '#' => self.tokenize_comment(),
                _ => panic!("Invalid symbol: {:?}", &self.cur_char),
            },
            None => todo!(),
        };
        self.next_char();
        ret
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

    fn tokenize_comment(&mut self) -> Token {
        self.next_char();
        while let Some(cur_ch) = self.cur_char {
            if cur_ch != '\n' && cur_ch != '#' {
                self.next_char();
            } else {
                break;
            }
        }
        self.next_char();
        self.tokenize()
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_alphanumeric() {
            self.next_char();
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
                while ch.is_whitespace() {
                    self.next_char();
                    match self.cur_char {
                        Some(cch) => ch = cch,
                        None => return,
                    }
                }
            }
            None => return,
        }
    }
}
