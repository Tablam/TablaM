use crate::types::DataType;

/// Define the internal errors
#[derive(Debug)]
pub enum Error {
    TypeMismatch { expected: DataType, get: DataType },
}

pub type ResultT<T> = Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    NotFound,
    Invalid,
    Duplicated,
    Parse,
    OutOfBounds,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCtx {
    key: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorLoc {
    source: String,
    loc: String,
}

/// Define the main Error type for the language
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorLang {
    tag: usize,
    kind: ErrorKind,
    msg: Option<String>,
    context: Option<Vec<ErrorCtx>>,
    source: Option<Vec<ErrorLoc>>,
}
