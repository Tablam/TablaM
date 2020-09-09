use std::fmt;

use crate::for_impl::*;
use crate::prelude::*;
use crate::row::fmt_row;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SumVariant {
    // A single value like Color.Blue, Color.Some(1)
    One(String, Scalar),
    // A list of values like Color(1, 2, 3)
    Value(String, Vec<Scalar>),
    // A list of named values like Color(red=1, green=2, blue=3)
    Record(String, RelTuple),
}

impl SumVariant {
    pub fn some(value: Scalar) -> Self {
        Self::One("Some".into(), value)
    }

    pub fn none() -> Self {
        Self::One("None".into(), Scalar::Unit)
    }

    pub fn tag(&self) -> &str {
        match self {
            SumVariant::One(x, _) => x,
            SumVariant::Value(x, _) => x,
            SumVariant::Record(x, _) => x,
        }
    }

    pub fn first(&self) -> &Scalar {
        match self {
            SumVariant::One(_, x) => x,
            SumVariant::Value(_, x) => &x[0],
            SumVariant::Record(_, x) => x.data.values().next().unwrap(),
        }
    }
    pub fn kind(&self) -> DataType {
        match self {
            SumVariant::One(_, x) => DataType::Sum(vec![x.kind()].into()),
            SumVariant::Value(_, x) => {
                let kind: Vec<_> = x.iter().map(|x| x.kind()).collect();
                DataType::Sum(kind.into())
            }
            SumVariant::Record(_, x) => x.kind(),
        }
    }
}

impl<T: Into<Scalar>> From<Option<T>> for SumVariant {
    fn from(x: Option<T>) -> Self {
        if let Some(x) = x {
            SumVariant::some(x.into())
        } else {
            SumVariant::none()
        }
    }
}

impl<T> From<SumVariant> for Option<T>
where
    T: From<Scalar>,
    T: From<Box<Scalar>>,
{
    fn from(x: SumVariant) -> Self {
        match x.tag() {
            "Some" => Some(x.first().clone().into()),
            "None" => None,
            x => unreachable!(x),
        }
    }
}

impl fmt::Display for SumVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SumVariant::One(tag, value) => write!(f, "{}({})", tag, value),
            SumVariant::Value(tag, value) => {
                write!(f, "{}(", tag)?;
                fmt_row(value, f)?;
                write!(f, ")")
            }
            SumVariant::Record(tag, value) => write!(f, "{}({})", tag, value),
        }
    }
}
