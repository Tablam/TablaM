use corelib::prelude::Span;
use parser::ast::Ast;

pub struct Program {}

impl Program {
    pub fn new(_source: &str) -> Self {
        Program {}
    }
    pub fn execute_str(&mut self, _source: &str) -> Ast {
        Ast::Eof(Span::from(token_eof()))
    }
}
