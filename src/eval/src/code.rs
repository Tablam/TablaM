use crate::env::Env;
use corelib::prelude::{Scalar, Span};
use parser::ast::Ast;

pub type CodeEx = Box<dyn FnMut(&Env) -> Code>;

/// Encode the executable code for the language using closures,
/// equivalent to bytecode
pub enum Code {
    Root,
    Scalar { val: Scalar, span: Span },
    If { code: CodeEx, span: Span },
    Eof,
}

pub fn compile(ast: &Parsed) -> Result<Code, ()> {
    // Only compile valid code!
    if !ast.p.errors.is_empty() {
        return Err(());
    }
    // Moving forward this MUST be correct code!

    Ok(Code::Eof)
}
