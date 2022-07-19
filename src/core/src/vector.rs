//! # Relational Vector.
//!
//! A [Vector] in TablaM can be considered a relation of exactly one column and *N* rows; so
//! it means that we can operate on it with all the relational/array operators.
//!
//! It also have the semantics of a "Column" so **each value is a row**.
//!
//! It also is the encoding of `structs`-like declarations
//!
use crate::prelude::*;
use crate::scalar::ScalarSlice;
use std::rc::Rc;

/// A struct used for indexing into a [Vector].
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct VecPos(usize, usize);

impl VecPos {
    /// Construct a new VecPos.
    pub fn new(column: usize, row: usize) -> Self {
        Self(column, row)
    }

    /// Get the column (x) index.
    pub fn column(&self) -> usize {
        self.0
    }

    /// Get the row (y) index.
    pub fn row(&self) -> usize {
        self.1
    }
}

impl From<(usize, usize)> for VecPos {
    fn from((column, row): (usize, usize)) -> Self {
        VecPos::new(column, row)
    }
}

impl std::fmt::Display for VecPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column(), self.row())
    }
}

/// The internal data for the [Vector]. We specialize for basic types only
/// and let [Scalar] handle the other cases.
//NOTE: This define a total order, so it matter what is the order of the enum!
//Must match DataType/Scalar
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Array {
    Bool(bv::BitVec),
    //Numeric
    I64(Vec<i64>),
    Decimal(Vec<Decimal>),
    F64(Vec<R64>),
    //Date
    Date(DateKind, Vec<DateT>),
    //Strings
    Utf8(Vec<String>),
    //Others
    Scalar(Vec<Scalar>),
}

impl Array {
    pub fn slice(&self) -> ScalarSlice<'_> {
        match &self {
            Array::Bool(x) => ScalarSlice::Bool(x),
            Array::I64(x) => ScalarSlice::I64(x),
            Array::Decimal(x) => ScalarSlice::Decimal(x),
            Array::F64(x) => ScalarSlice::F64(x),
            Array::Date(kind, x) => ScalarSlice::Date(*kind, x),
            Array::Utf8(x) => ScalarSlice::Utf8(x),
            Array::Scalar(x) => ScalarSlice::Scalar(x),
        }
    }
}

/// And 2d vector stored by *row-major*.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    /// An user-assigned schema
    pub schema: Rc<Schema>,
    pub rows: usize,
    pub data: Array,
}

impl Vector {
    pub fn new(schema: Schema, rows: usize, data: Array) -> Self {
        Self {
            rows,
            data,
            schema: Rc::new(schema),
        }
    }

    pub fn row(&self, row: usize) -> ScalarSlice<'_> {
        self.data.slice().to_row(row, self.schema.len())
    }
}

impl Rel for Vector {
    fn type_name(&self) -> &str {
        "Vector"
    }

    fn schema(&self) -> SchemaInfo {
        SchemaInfo::vec(&self.schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let nums = dsl::vector(1);
        assert_eq!(nums.rows, 1);

        dbg!(nums);
    }
}
