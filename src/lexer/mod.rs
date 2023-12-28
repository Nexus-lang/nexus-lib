pub mod tokens;

use clutils::files::FileHandler;

use self::tokens::Token;

pub struct Lexer {
    filehandler: FileHandler,
    cur_char: Option<char>,
    cur_pos: usize,
    next_pos: usize,
}

impl Lexer {
    pub fn new(path: String) -> Self {
        let filehandler = FileHandler::new(path);
        let mut lexer = Self {
            filehandler: filehandler,
            cur_char: None,
            cur_pos: 0,
            next_pos: 0,
        };
        lexer.next_char();
        lexer
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
        Token::Num(string.parse().unwrap())
    }

    fn tokenize_symbol(&mut self) -> Token {
        let ret = match self.cur_char {
            Some(ch) => match ch {
                '=' => match self.filehandler.content.chars().nth(self.next_pos) {
                    Some('=') => {
                        self.next_char();
                        Token::Equals
                    }
                    _ => Token::Assign,
                },
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                ';' => todo!(),
                '(' => Token::LParent,
                ')' => Token::RParent,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                '"' => {
                    self.next_char();
                    let first_pos = self.cur_pos;
                    while self.cur_char != Some('"') {
                        self.next_char();
                    }
                    let string: String = self.filehandler.content[first_pos..self.cur_pos].into();
                    Token::String(string.chars().collect(), None)
                }
                ':' => Token::Colon,
                ',' => Token::Comma,
                '#' => {
                    let first_pos = self.next_pos;
                    while self.filehandler.content.chars().nth(self.next_pos) != Some('\n') {
                        if self.cur_pos + 1 != self.filehandler.content.len() {
                            self.next_char();
                        } else {
                            return Token::Eof;
                        }
                    }
                    let string = self.filehandler.content[first_pos..self.cur_pos].into();
                    Token::Comment(string)
                }
                _ => panic!("Invalid symbol: {:?}", &self.cur_char),
            },
            None => todo!(),
        };
        self.next_char();
        ret
    }

    fn tokenize_ident(&mut self) -> Token {
        let first_pos = self.cur_pos;
        while self.cur_char.is_some() && self.cur_char.unwrap().is_alphanumeric() {
            self.next_char();
        }
        let string: String = self.filehandler.content[first_pos..self.cur_pos].into();
        return match string.as_str() {
            "var" => Token::Var,
            "const" => Token::Const,
            "loop" => Token::Loop,
            "func" => Token::Func,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "break" => Token::Break,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "and" => Token::And,
            "or" => Token::Or,
            "when" => Token::When,
            "use" => Token::When,

            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            _ => {
                    Token::Ident(self.filehandler.content[first_pos..self.cur_pos].into())
            }
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