pub mod ast;
pub mod env;
pub mod modules;
pub mod program;

pub type Identifier = String;

pub mod prelude {
    pub use super::ast::*;
    pub use super::modules::*;
}
