use crate::checklist::{CheckError, CheckList, Step};
use crate::cst::CstNode;
use crate::token::Token;
use corelib::errors::Span;

/// Define the main Error type for the parser
#[derive(Debug, Clone)]
pub enum ErrorParser {
    BoolExpr { span: Span, found: String },
    NoExpr { span: Span, found: String },
    ScalarParse { span: Span, msg: String },
    Incomplete { err: CheckError, missing: Vec<Step> },
}

impl From<CheckError> for ErrorParser {
    fn from(err: CheckError) -> Self {
        let missing = err.expect.into_iter().collect();
        ErrorParser::Incomplete { err, missing }
    }
}

pub(crate) fn parse(t: &Token, msg: &str) -> ErrorParser {
    let span: Span = t.into();
    ErrorParser::ScalarParse {
        span,
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
