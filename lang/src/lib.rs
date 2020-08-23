pub mod ast;
pub mod eval;
pub mod lexer;
pub mod scanner;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::ast::{Error, Return};
    pub use crate::lexer::Token;
}
