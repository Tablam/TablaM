use std::hash::Hash;
use std::rc::Rc;

use decorum::R64;
use derive_more::{Display, From};
use rust_decimal::Decimal;

use crate::types::{DataType, NativeKind};

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
}

impl Scalar {
    fn type_name(&self) -> &str {
        match self {
            Scalar::None => "None",
            Scalar::Bit(_) => "Bit",
            Scalar::Bool(_) => "Bool",
            Scalar::I64(_) => "I64",
            Scalar::F64(_) => "F64",
            Scalar::Decimal(_) => "Decimal",
            Scalar::Time(_) => "Time",
            Scalar::Date(_) => "Date",
            Scalar::Char(_) => "Char",
            Scalar::DateTime(_) => "DateTime",
            Scalar::UTF8(_) => "UTF8",
        }
    }

    fn kind(&self) -> DataType {
        match self {
            Scalar::None => DataType::None,
            Scalar::Bool(_) => DataType::Bool,
            Scalar::Bit(_) => DataType::Bool,
            Scalar::I64(_) => DataType::I64,
            Scalar::F64(_) => DataType::F64,
            Scalar::Decimal(_) => DataType::Decimal,
            Scalar::Time(_) => DataType::Time,
            Scalar::Date(_) => DataType::Date,
            Scalar::DateTime(_) => DataType::DateTime,
            Scalar::Char(_) => DataType::DateTime,
            Scalar::UTF8(_) => DataType::UTF8,
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

kind_native!(bool, Bool);
kind_native!(Decimal, Decimal);
kind_native!(R64, F64);
kind_native!(String, UTF8);

impl From<&str> for Scalar {
    fn from(x: &str) -> Self {
        Scalar::UTF8(Rc::new(x.into()))
    }
}
