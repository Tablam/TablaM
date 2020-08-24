pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::lexer::Token;
}
