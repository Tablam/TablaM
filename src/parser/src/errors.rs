use crate::checklist::{CheckError, CheckList, Step};
use crate::cst::CstNode;
use crate::token::Token;
use corelib::errors::Span;
use corelib::types::DataType;

pub enum ErrorCode {
    ParseLiteral = 1,
}

/// Define the main Error type for the parser
#[derive(Debug, Clone)]
pub enum ErrorParser {
    BoolExpr {
        span: Span,
        found: String,
    },
    NoExpr {
        span: Span,
        found: String,
    },
    ScalarParse {
        span: Span,
        kind: DataType,
        msg: String,
    },
    Incomplete {
        err: CheckError,
        missing: Vec<Step>,
    },
}

impl ErrorParser {
    pub fn error_code(&self) -> ErrorCode {
        match self {
            ErrorParser::ScalarParse { .. } => ErrorCode::ParseLiteral,
            _ => todo!(),
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            ErrorParser::BoolExpr { span, .. } => span,
            ErrorParser::NoExpr { span, .. } => span,
            ErrorParser::ScalarParse { span, .. } => span,
            ErrorParser::Incomplete { err, .. } => &err.span,
        }
    }
}

impl From<CheckError> for ErrorParser {
    fn from(err: CheckError) -> Self {
        let missing = err.expect.into_iter().collect();
        ErrorParser::Incomplete { err, missing }
    }
}

pub(crate) fn parse(t: &Token, kind: DataType, msg: &str) -> ErrorParser {
    let span: Span = t.into();
    ErrorParser::ScalarParse {
        span,
        kind,
        msg: msg.into(),
    }
}

pub(crate) fn not_a_expr(t: &Token, code: &str) -> ErrorParser {
    let span: Span = t.into();
    ErrorParser::NoExpr {
        span,
        found: code.into(),
    }
}

pub(crate) fn bool_expr(t: &Token, code: &str) -> ErrorParser {
    let span: Span = t.into();
    ErrorParser::BoolExpr {
        span,
        found: code.into(),
    }
}

pub(crate) fn incomplete(t: &CheckList, found: CstNode) -> ErrorParser {
    let span: Span = t.span();
    let missing = t.expect.into_iter().collect();
    let err = CheckError {
        span,
        found,
        expect: None,
    };
    ErrorParser::Incomplete { err, missing }
}
