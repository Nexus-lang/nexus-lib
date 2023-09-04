use crate::{tokens::{Token, TokenType}, ast::{Expression, Statement}};

// TODO: simplify error messages for beginners

/// Cannot declare `{}` as local
pub fn invalid_local_decl(token: &Token) -> String {
    format!("Cannot declare `{}` as local", token.literal)
}

/// Expected statement on the left of local declaration. Got illegal statement {:?} instead
pub fn empty_local_decl(left: &Statement) -> String {
    format!("Expected statement on the left side of the local declaration. Got {:?} instead", left)
}

/// Expected value on the left side of the assign. Got {:?} instead
pub fn empty_variable_val(value: &Expression) -> String {
    format!("Expected value on the left side of the assign. Got {:?} instead", value)
}

/// Expected value on the left side of the return. Got {:?} instead
pub fn empty_return_val(value: &Expression) -> String {
    format!("Expected value on the left side of the return. Got {:?} instead", value)
}

/// Expected value on the left side of the return. Got {:?} instead
pub fn empty_condition(expression_type: &TokenType, value: &Expression) -> String {
    format!("Expected condition left side of the {}. Got {:?} instead", expression_type.literal(), value)
}

/// Expected identifier like `x` found {} instead
pub fn invalid_identifier(ident: &String) -> String {
    format!("Expected identifier like `x` found {} instead", ident)
}