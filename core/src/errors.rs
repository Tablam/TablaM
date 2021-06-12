use crate::function::FunctionDec;
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
    IoError(std::io::Error),
    FileIoError(FileError),
    #[display(fmt = "File Error: {}", _0)]
    FileError(std::io::Error, String),
    RankNotMatch,
    #[display(fmt = "Type mismatch {} <> {}", _0, _1)]
    TypeMismatchBinOp(DataType, DataType),
    #[display(fmt = "Type mismatch in the function {} params", _0)]
    ParamTypeMismatch(String),
    #[display(fmt = "The function was called with {} params when need {}", _0, _1)]
    ParamCount(usize, usize),
    #[display(fmt = "The function {}.name is not found", _0)]
    FunctionNotFound(FunctionDec),
}

impl Error {
    pub fn file_err(e: std::io::Error, path: std::path::PathBuf) -> Self {
        Error::FileIoError(FileError::new(e, path))
    }
}

pub type ResultT<T> = std::result::Result<T, Error>;
