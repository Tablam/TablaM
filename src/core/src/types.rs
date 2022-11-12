use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::hash::Hash;

use crate::prelude::DateT;
use crate::scalar::DateKind;

//Type Alias...
pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
pub type Date = chrono::Date<chrono::FixedOffset>;
pub type Time = chrono::NaiveTime;
pub type FileId = tree_flat::node::NodeId;

/// This trait help to mark values that are compatible with the semantics of the relational model
pub trait Value: Clone + PartialEq + PartialOrd + Eq + Ord + Hash {}
impl<T: Clone + PartialEq + PartialOrd + Eq + Ord + Hash> Value for T {}

//NOTE: This defines a total order, so it matter what is the order of the enum!
//Must match Scalar
//The overall sorting order is defined as:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataType {
    //The BOTTOM type
    Unit,
    Bool,
    Bit,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arity {
    Scalar,
    Vector,
    Table,
}

pub const DATE_FMT: &str = "%Y-%m-%d";
pub const TIME_FMT: &str = "%H:%M:%S";
pub const DATE_TIME_FMT: &str = "%Y-%m-%d %H:%M:%S %z";

fn to_date(of: NaiveDateTime) -> chrono::DateTime<FixedOffset> {
    let of = chrono::DateTime::<Utc>::from_utc(of, Utc);
    DateTime::from(of)
}

pub fn parse_date_t(of: &str) -> Result<DateT, chrono::ParseError> {
    let of = NaiveDate::parse_from_str(of, DATE_FMT)?.and_hms(0, 0, 0);

    Ok(DateT::date(to_date(of)))
}

pub fn parse_time_t(of: &str) -> Result<DateT, chrono::ParseError> {
    let d = NaiveTime::parse_from_str(of, TIME_FMT)?;
    let d = chrono::naive::NaiveDate::MIN.and_time(d);

    Ok(DateT::time(to_date(d)))
}

pub fn parse_date_time_t(of: &str) -> Result<DateT, chrono::ParseError> {
    let d = DateTime::parse_from_str(of, DATE_TIME_FMT)?;

    Ok(DateT::datetime(d))
}
