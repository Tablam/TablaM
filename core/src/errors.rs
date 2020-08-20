use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "The schemas must match exactly (field count, names & types)")]
    SchemaNotMatchExact,
    #[display(fmt = "IO Error: {}", _0)]
    IOError(std::io::Error),
}
