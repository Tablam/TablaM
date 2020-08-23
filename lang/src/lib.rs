pub mod ast;
pub mod lexer;
pub mod scanner;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::ast::{Error, Result};
    pub use crate::lexer::Token;
}
