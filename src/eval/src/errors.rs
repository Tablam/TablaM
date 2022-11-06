use corelib::prelude::Span;
use parser::errors::ErrorParser;

#[derive(Debug, Clone)]
pub enum ErrorCode {
    Parser { error: ErrorParser },
}

impl ErrorCode {
    pub fn span(&self) -> &Span {
        match self {
            ErrorCode::Parser { error } => error.span(),
        }
    }
}

impl From<&[ErrorParser]> for ErrorCode {
    fn from(x: &[ErrorParser]) -> Self {
        let x = x.first().unwrap();

        ErrorCode::Parser { error: x.clone() }
    }
}
