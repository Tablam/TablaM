use crate::checklist::{CheckList, Step};
use crate::token::Token;
use corelib::errors::{ErrorKind, Span};

#[derive(Debug, Clone)]
pub enum Error {
    Simple,
    Incomplete(Vec<Step>),
}

/// Define the main Error type for the parser
#[derive(Debug, Clone)]
pub struct ErrorParser {
    kind: ErrorKind,
    msg: Option<String>,
    span: Span,
    error: Error,
}

impl ErrorParser {
    pub fn new(kind: ErrorKind, msg: Option<&str>, span: Span, error: Error) -> Self {
        Self {
            kind,
            msg: msg.map(|x| x.to_string()),
            span,
            error,
        }
    }
}

impl From<(Span, Step)> for ErrorParser {
    fn from(e: (Span, Step)) -> Self {
        ErrorParser::new(ErrorKind::Parse, None, e.0, Error::Incomplete(vec![e.1]))
    }
}

pub fn parse(t: &Token, msg: &str) -> ErrorParser {
    let span: Span = t.into();
    ErrorParser::new(ErrorKind::Parse, Some(msg), span, Error::Simple)
}

pub fn incomplete(t: &CheckList) -> ErrorParser {
    let span: Span = t.span();
    ErrorParser::new(
        ErrorKind::Parse,
        None,
        span,
        Error::Incomplete(t.pending().into()),
    )
}
