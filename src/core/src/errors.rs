use crate::types::{DataType, FileId};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use text_size::TextRange;

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
    Incomplete,
    OutOfBounds,
    Unauthorized,
    Forbidden,
    NotAllowed,
    Timeout,
    Conflict,
    NotImplemented,
    ServerError,
    Unavailable,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeCode(pub TextRange);

impl PartialOrd for RangeCode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for RangeCode {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl Hash for RangeCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl From<TextRange> for RangeCode {
    fn from(x: TextRange) -> Self {
        RangeCode(x)
    }
}

impl fmt::Display for RangeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub file_id: FileId,
    pub range: RangeCode,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCtx {
    key: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorLoc {
    Custom { source: String, loc: String },
    Code(Span),
}

/// Define the main Error type for the language
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorLang {
    kind: ErrorKind,
    msg: Option<String>,
    context: Option<Vec<ErrorCtx>>,
    source: Option<Vec<ErrorLoc>>,
}

impl ErrorLang {
    pub fn new(kind: ErrorKind, msg: Option<&str>) -> Self {
        Self {
            kind,
            msg: msg.map(|x| x.to_string()),
            context: None,
            source: None,
        }
    }

    pub fn with_span(self, of: Span) -> Self {
        let mut x = self;
        if let Some(ref mut s) = x.source {
            s.push(ErrorLoc::Code(of))
        } else {
            x.source = Some(vec![ErrorLoc::Code(of)])
        }
        x
    }

    pub fn with_ctx(self, of: ErrorCtx) -> Self {
        let mut x = self;
        if let Some(ref mut s) = x.context {
            s.push(of)
        } else {
            x.context = Some(vec![of])
        }
        x
    }
}

impl fmt::Display for ErrorLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}Error", self.kind)?;
        if let Some(msg) = &self.msg {
            writeln!(f, ": \"{}\"", msg)?;
        }
        if let Some(err) = self.source.as_deref() {
            for e in err {
                match e {
                    ErrorLoc::Custom { source, loc } => {
                        writeln!(f, " Source: {}", source)?;
                        writeln!(f, "  at: {}", loc)?;
                    }
                    ErrorLoc::Code(x) => {
                        writeln!(f, " Line {} Col {}", x.line, x.col)?;
                        writeln!(f, "  at: {}", x.file_id)?;
                    }
                }
            }
        }
        if let Some(err) = self.context.as_deref() {
            writeln!(f, " Context:")?;
            for e in err {
                writeln!(f, " {}: {}", e.key, e.value)?;
            }
        }
        Ok(())
    }
}
