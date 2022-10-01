use corelib::prelude::Span;
use parser::errors::ErrorParser;

#[derive(Debug)]
pub enum ErrorCode {
    Parser {
        errors: Vec<ErrorParser>,
        span: Span,
    },
}

impl ErrorCode {
    pub fn span(&self) -> &Span {
        match self {
            ErrorCode::Parser { span, .. } => span,
        }
    }
}

impl From<&[ErrorParser]> for ErrorCode {
    fn from(x: &[ErrorParser]) -> Self {
        let span = x.first().map(|x| x.span()).unwrap();

        ErrorCode::Parser {
            errors: x.into(),
            span: span.clone(),
        }
    }
}
