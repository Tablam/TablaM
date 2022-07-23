use crate::ast::Expr;

pub struct Program {}

impl Program {
    pub fn new(_source: &str) -> Self {
        Program {}
    }
    pub fn execute_str(&mut self, _source: &str) -> Expr {
        Expr::Eof(0)
    }
}
