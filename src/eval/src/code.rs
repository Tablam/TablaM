use crate::env::Env;
use crate::errors::ErrorCode;
use corelib::prelude::{Scalar, Span};
use corelib::tree_flat::prelude::Tree;
use parser::ast::Ty;
use std::fmt;

pub type CodeEx = Box<dyn FnMut(&Env) -> Code>;
pub type CodeId = usize;

/// Encode the executable code for the language using closures,
/// equivalent to bytecode
#[derive(Debug)]
pub enum Code {
    Root,
    Scalar { val: Scalar, span: Span },
    If { code: CodeId, span: Span },
    Halt { error: ErrorCode, span: Span },
    Eof,
}

pub struct CodePrinter<'a> {
    pub(crate) parsed: &'a Tree<Code>,
    pub(crate) src: &'a str,
}

fn fmt_plain<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    level: usize,
    val: &T,
    span: &Span,
) -> fmt::Result {
    write!(f, "{}{}: {:?}", " ".repeat(level + 1), span.range, val)
}

pub(crate) fn fmt_t<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    level: usize,
    val: &T,
    span: &Span,
) -> fmt::Result {
    write!(f, "{} @@ {}: {:?}", " ".repeat(level + 1), span.range, val)
}

fn fmt_node(node: &Code, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match node {
        Code::Root => write!(f, "Root")?,
        Code::Scalar { val, span } => fmt_t(f, level, val, span)?,
        Code::If { code, span } => {
            fmt_plain(f, level, &"if", span)?;
        }
        Code::Halt { error, span } => {
            fmt_plain(f, level, &"ERROR", span)?;
        }
        Code::Eof => write!(f, "Eof")?,
    }

    Ok(())
}

impl fmt::Display for CodePrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.parsed.iter() {
            fmt_node(node.data, node.level(), f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
