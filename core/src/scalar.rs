use std::hash::Hash;
use std::rc::Rc;

use decorum::R64;
use derive_more::{Display, From};
use rust_decimal::Decimal;

use crate::sum_type::Case;
use crate::types::{DataType, NativeKind, Rel};
use crate::vector::Vector;

pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
pub type Date = chrono::Date<chrono::FixedOffset>;
pub type Time = chrono::NaiveTime;

//NOTE: The order of this enum must match DataType
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From)]
pub enum Scalar {
    None,
    Bit(u8),
    Bool(bool),
    //Numeric
    I64(i64),
    F64(R64),
    Decimal(Decimal),
    //Date
    Time(Time),
    Date(Date),
    DateTime(DateTime),
    //Strings
    Char(char),
    UTF8(Rc<String>),
    //Sum types
    Sum(Box<Case>),
    //Collections
    Vector(Box<Vector>),
    //Lazy computation
    //Seq(Seq<'static>),
}

impl Scalar {
    pub fn kind(&self) -> DataType {
        match self {
            Scalar::None => DataType::None,
            Scalar::Bit(_) => DataType::Bit,
            Scalar::Bool(_) => DataType::Bool,
            Scalar::Char(_) => DataType::Char,
            Scalar::Date(_) => DataType::Date,
            Scalar::DateTime(_) => DataType::DateTime,
            Scalar::Decimal(_) => DataType::Decimal,
            Scalar::F64(_) => DataType::F64,
            Scalar::I64(_) => DataType::I64,
            Scalar::Time(_) => DataType::Time,
            Scalar::UTF8(_) => DataType::UTF8,
            Scalar::Sum(x) => DataType::Sum(Box::new(x.value.kind())),
            Scalar::Vector(x) => x.kind(),
            //Scalar::Seq(x) => {x.kind()}
        }
    }
}

//Type Alias...
pub type Col = Vec<Scalar>;

macro_rules! kind_native {
    ($native:ident, $kind:ident) => {
        impl NativeKind for $native {
            fn kind() -> DataType {
                DataType::$kind
            }
        }
    };
}

kind_native!(i64, I64);
kind_native!(bool, Bool);
kind_native!(Decimal, Decimal);
kind_native!(R64, F64);
kind_native!(String, UTF8);

impl From<i32> for Scalar {
    fn from(x: i32) -> Self {
        Scalar::I64(x as i64)
    }
}

impl From<&str> for Scalar {
    fn from(x: &str) -> Self {
        Scalar::UTF8(Rc::new(x.into()))
    }
}

impl From<Box<Scalar>> for Scalar {
    fn from(x: Box<Scalar>) -> Self {
        *x
    }
}

impl From<Case> for Scalar {
    fn from(x: Case) -> Self {
        Scalar::Sum(Box::new(x))
    }
}

impl From<Vector> for Scalar {
    fn from(x: Vector) -> Self {
        Scalar::Vector(Box::new(x))
    }
}
