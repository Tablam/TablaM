extern crate enum_map;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate console;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod modules;
pub mod parser;
pub mod stdlib;

mod for_impl {
    pub use std::collections::HashMap;
    pub use std::fmt;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
}

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::lexer::{Token, TokenData};
    pub use crate::modules;
    pub use crate::stdlib;
}
