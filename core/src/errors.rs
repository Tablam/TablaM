use derive_more::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum RelError {
    #[display(fmt = "Schemas names & types not match")]
    SchemaNotMatchExact,
}
