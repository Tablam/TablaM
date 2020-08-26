use crate::types::DataType;
use derive_more::{Display, From};

#[derive(Debug, From, Display)]
#[display(fmt = "'{:?}' Error: {}", path, e)]
pub struct FileError {
    e: std::io::Error,
    path: std::path::PathBuf,
}

impl FileError {
    pub fn new(e: std::io::Error, path: std::path::PathBuf) -> Self {
        FileError { e, path }
    }
}

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "The schemas must match exactly (field count, names & types)")]
    SchemaNotMatchExact,
    #[display(fmt = "IO Error: {}", _0)]
    IOError(std::io::Error),
    FileIOError(FileError),
    #[display(fmt = "File Error: {}", _0)]
    FileError(std::io::Error, String),
    RankNotMatch,
    #[display(fmt = "Type mismatch {} <> {}", _0, _1)]
    TypeMismatchBinOp(DataType, DataType),
    #[display(fmt = "The function was called with {} params when need {}", _0, _1)]
    ParamCount(usize, usize),
}

impl Error {
    pub fn file_err(e: std::io::Error, path: std::path::PathBuf) -> Self {
        Error::FileIOError(FileError::new(e, path))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
