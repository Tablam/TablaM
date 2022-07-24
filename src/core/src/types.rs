use crate::scalar::DateKind;
use std::hash::Hash;

//Type Alias...
pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
pub type Date = chrono::Date<chrono::FixedOffset>;
pub type Time = chrono::NaiveTime;
pub type FileId = tree_flat::node::NodeId;

/// This trait help to mark values that are compatible with the semantics of the relational model
pub trait Value: Clone + PartialEq + PartialOrd + Eq + Ord + Hash {}
impl<T: Clone + PartialEq + PartialOrd + Eq + Ord + Hash> Value for T {}

//NOTE: This define a total order, so it matter what is the order of the enum!
//Must match Scalar
//The overall sorting order is defined as:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataType {
    //The BOTTOM type
    Unit,
    Bool,
    // Numeric
    I64,
    Decimal,
    F64,
    // Dates
    Date(DateKind),
    // Text
    Utf8,
    //The TOP type
    //For List, dynamic
    Any,
}

pub trait NativeKind {
    fn kind() -> DataType;
    fn num_rows() -> usize;
}
