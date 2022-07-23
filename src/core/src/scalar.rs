//! # Relational scalar.
//!
//! A [Scalar] in TablaM can be considered a relation of exactly one row, one column, one value; so
//! it means that we can operate on it with all the relational/array operators.

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DateKind {
    Time,
    Date,
    DateTime,
}

/// An unified Date structure that collapse the different [DateKind] in a single value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateT {
    pub kind: DateKind,
    pub date: DateTime,
}

pub type BoolBit = bv::BitArr!(for 1, in usize, bv::Lsb0);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarSlice<'a> {
    /// The **BOTTOM** value
    Unit(&'a [()]),
    Bool(&'a bv::BitSlice),
    //Numeric
    I64(&'a [i64]),
    Decimal(&'a [Decimal]),
    F64(&'a [R64]),
    //Date
    Date(DateKind, &'a [DateT]),
    //Strings
    Utf8(&'a [String]),
    // General
    Scalar(&'a [Scalar]),
    /// The **TOP** value
    Top(&'a [()]),
}

impl<'a> ScalarSlice<'a> {
    fn len(&self) -> usize {
        match self {
            Self::Unit(x) => x.len(),
            Self::Bool(x) => x.len(),
            Self::I64(x) => x.len(),
            Self::Decimal(x) => x.len(),
            Self::F64(x) => x.len(),
            Self::Date(_, x) => x.len(),
            Self::Utf8(x) => x.len(),
            Self::Scalar(x) => x.len(),
            Self::Top(x) => x.len(),
        }
    }

    pub fn kind(&self) -> DataType {
        match self {
            Self::Unit(_) => DataType::Unit,
            Self::Bool(_) => DataType::Bool,
            Self::I64(_) => DataType::I64,
            Self::Decimal(_) => DataType::Decimal,
            Self::F64(_) => DataType::F64,
            Self::Date(x, _) => DataType::Date(*x),
            Self::Utf8(_) => DataType::Utf8,
            Self::Scalar(_) => DataType::Any,
            Self::Top(_) => DataType::Any,
        }
    }

    pub(crate) fn range(&self, r: Range<usize>) -> Self {
        match self {
            Self::Unit(x) => Self::Unit(&x[r]),
            Self::Bool(x) => Self::Bool(&x[r]),
            Self::I64(x) => Self::I64(&x[r]),
            Self::Decimal(x) => Self::Decimal(&x[r]),
            Self::F64(x) => Self::F64(&x[r]),
            Self::Date(kind, x) => Self::Date(*kind, &x[r]),
            Self::Utf8(x) => Self::Utf8(&x[r]),
            Self::Scalar(x) => Self::Scalar(&x[r]),
            Self::Top(x) => Self::Top(&x[r]),
        }
    }

    /// Get a row range
    pub fn to_row(self, row: usize, cols: usize) -> Self {
        self.range(row * cols..cols)
    }
}

/// The scalar values stored as [T;1] to make easier to see them as rows/slices
//NOTE: This define a total order, so it matter what is the order of the enum!
//Must match DataType
//The overall sorting order is defined as:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scalar {
    /// The **BOTTOM** value
    Unit([(); 1]),
    Bool(BoolBit),
    //Numeric
    I64([i64; 1]),
    Decimal([Decimal; 1]),
    F64([R64; 1]),
    //Date
    Date([DateT; 1]),
    //Strings
    Utf8([String; 1]),
    /// The **TOP** value
    Top([(); 1]),
}

impl Scalar {
    pub fn kind(&self) -> DataType {
        self.slice().kind()
    }

    pub fn slice(&self) -> ScalarSlice<'_> {
        match self {
            Self::Unit(x) => ScalarSlice::Unit(x),
            Self::Bool(x) => ScalarSlice::Bool(x),
            Self::I64(x) => ScalarSlice::I64(x),
            Self::Decimal(x) => ScalarSlice::Decimal(x),
            Self::F64(x) => ScalarSlice::F64(x),
            Self::Date(x) => ScalarSlice::Date(x[0].kind, x),
            Self::Utf8(x) => ScalarSlice::Utf8(x),
            Self::Top(x) => ScalarSlice::Top(x),
        }
    }
}

pub struct SchemaScalar {}

impl Rel for Scalar {
    fn type_name(&self) -> &str {
        "Scalar"
    }

    fn schema(&self) -> SchemaInfo {
        let kind = self.slice().kind();
        SchemaInfo::scalar(kind)
    }
}
