use parser::ast::{OptionallyTypedIdent, BlockStmt};

pub fn typed_vec_to_string(val: &Vec<OptionallyTypedIdent>) -> String {
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