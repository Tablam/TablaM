use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::ops::Range;

use text_size::TextRange;

use crate::types::{DataType, FileId};

/// Define the internal errors
#[derive(Debug)]
pub enum ErrorCore {
    TypeMismatch { expected: DataType, get: DataType },
}

pub type ResultT<T> = Result<T, ErrorCore>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangeCode(pub TextRange);

impl PartialOrd for RangeCode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.len().cmp(&other.0.len()))
    }
}

impl Ord for RangeCode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.len().cmp(&other.0.len())
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

impl Span {
    pub fn range(&self) -> Range<usize> {
        let start: usize = self.range.0.start().into();
        let end: usize = self.range.0.end().into();

        start..end
    }
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
    /// Optional context for the Error: Which record was not found, what value was invalid, etc.
    context: Option<Vec<ErrorCtx>>,
    /// Optional location of the Error in the source code
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
