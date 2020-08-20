use crate::scalar::Scalar;
use derive_more::{Display, From};
use std::collections::HashMap;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Schemas names & types not match")]
    SchemaNotMatchExact,
    #[display(fmt = "IO Error: {}", _0)]
    IOError(std::io::Error),
}
