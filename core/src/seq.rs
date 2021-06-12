use crate::for_impl::*;
use crate::prelude::*;

use derivative::Derivative;
use derive_more::Display;

pub type RelIter = dyn for<'a> Iterator<Item = Scalar>;

#[derive(Derivative, Display)]
#[derivative(Hash, PartialEq, PartialOrd, Ord, Eq)]
#[display(fmt = "Seq({})", schema)]
pub struct Seq {
    pub schema: Schema,
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    pub iter: Box<RelIter>,
}

impl Seq {
    pub fn new(schema: Schema, iter: Box<dyn Iterator<Item = Scalar>>) -> Self {
        Seq { schema, iter }
    }
}

impl fmt::Debug for Seq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
