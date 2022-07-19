use crate::ast::{Expr, Return};

pub struct Program {}

impl Program {
    pub fn new(_source: &str) -> Self {
        Program {}
    }
    pub fn execute_str(&mut self, _source: &str) -> Return {
        Ok(Expr::Eof(0))
    }
}
