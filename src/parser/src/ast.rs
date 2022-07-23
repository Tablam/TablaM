use crate::files::FileId;
use crate::token::Token;
use corelib::prelude::DataType;
use corelib::scalar::Scalar;
use std::fmt;
use text_size::TextRange;

pub type Return = std::result::Result<Ast, ()>;

#[derive(Debug, Clone)]
pub struct Span {
    pub file_id: FileId,
    pub range: TextRange,
    pub line: u32,
    pub col: u32,
}

impl From<&Token> for Span {
    fn from(x: &Token) -> Self {
        Span {
            file_id: x.file_id,
            range: x.range,
            line: x.line,
            col: x.col,
        }
    }
}

/// Encode the type definitions
#[derive(Debug, Clone)]
pub enum Ty {
    /// Not need a type, like a "pass" expression
    Ignore,
    /// Means is not yet know the type
    Unknown,
    /// A type from a Relation
    Kind(DataType),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Ignore => {}
            Ty::Unknown => {}
            Ty::Kind(x) => write!(f, "T: {:?}", x)?,
        }
        Ok(())
    }
}
/// Encode the parse-tolerant AST
#[derive(Debug, Clone)]
pub enum Ast {
    Root,
    Scalar { val: Scalar, span: Span },
    Pass(Span),
}

impl Ast {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Ast::Root => Ty::Ignore,
            Ast::Scalar { val, span } => Ty::Kind(val.kind()),
            Ast::Pass(_) => Ty::Ignore,
        }
    }

    pub(crate) fn scalar(val: Scalar, t: &Token) -> Self {
        Self::Scalar {
            val,
            span: t.into(),
        }
    }
}
