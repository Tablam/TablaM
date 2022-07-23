use crate::ast::Span;
use crate::token::Token;

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}

impl Error {
    pub fn new(token: &Token, msg: &str) -> Self {
        Self {
            span: token.into(),
            msg: msg.into(),
        }
    }
}
