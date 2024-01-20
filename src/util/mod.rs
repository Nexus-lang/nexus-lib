use crate::{parser::ast::{OptionallyTypedIdent, BlockStmt}, lexer::{Lexer, tokens::Token}};

pub fn get_next_tok(lexer: &mut Lexer) -> Token {
    loop {
        let tok = lexer.tokenize();
        if let Some(tok) = tok {
            return tok;
        }
    }
}

pub fn typed_vec_to_string(val: &[OptionallyTypedIdent]) -> String {
    let mut buf = String::new();
    val.iter().for_each(|i| {
        let str = format!("{}{}", &i.ident.0, match &i._type {
            Some(_type) => format!(": {}", &_type.0),
            None => String::new(),
        });
        buf.push_str(&str);
        buf.push(',')
    });
    buf.pop();
    buf
}

pub fn block_to_string(val: &BlockStmt) -> String {
    let mut buf = String::new();
    val.stmts.iter().for_each(|stmt| {
        // TODO: implement display for statement
        buf.push_str(&format!("    {:#?}\n", stmt))
    });
    buf
}
