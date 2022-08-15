use crate::token::{CmpOp, Token};
use corelib::errors::Span;
use corelib::prelude::DataType;
use corelib::scalar::Scalar;
use std::fmt;

pub type Return = std::result::Result<Ast, ()>;

impl From<&Token> for Span {
    fn from(x: &Token) -> Self {
        Span {
            file_id: x.file_id,
            range: x.range.into(),
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

#[derive(Debug, Clone)]
pub enum ExprBool {
    Scalar { val: Scalar, span: Span },
}

impl ExprBool {
    pub(crate) fn span(&self) -> Span {
        match self {
            ExprBool::Scalar { span, .. } => *span,
        }
    }

    pub(crate) fn bool(val: bool, t: &Token) -> Self {
        Self::Scalar {
            val: val.into(),
            span: t.into(),
        }
    }
}

/// Encode the parse-tolerant AST
#[derive(Debug, Clone)]
pub enum Ast {
    //Markers
    Root(Span),
    // AST productions
    Scalar {
        val: Scalar,
        span: Span,
    },
    IfBlock {
        if_span: Span,
        do_span: Span,
        else_span: Span,
        end_span: Span,
        check: Box<ExprBool>,
        if_true: Box<Ast>,
        if_false: Box<Ast>,
    },
    Bool {
        val: Scalar,
        span: Span,
    },
    Cmp {
        op: CmpOp,
        span: Span,
    },
    Pass(Span),
    Eof(Span),
}

impl Ast {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Ast::Root(_) => Ty::Ignore,
            Ast::Scalar { val, span: _ } => Ty::Kind(val.kind()),
            Ast::Pass(_) => Ty::Unknown,
            Ast::Eof(_) => Ty::Ignore,
            Ast::Bool { .. } => Ty::Kind(DataType::Bool),
            Ast::Cmp { .. } => Ty::Unknown,
            Ast::IfBlock { .. } => Ty::Unknown,
        }
    }

    pub(crate) fn scalar(val: Scalar, t: &Token) -> Self {
        Self::Scalar {
            val,
            span: t.into(),
        }
    }
}
