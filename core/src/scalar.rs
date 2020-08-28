use std::hash::Hash;
use std::rc::Rc;

use decorum::R64;
use derive_more::{Display, From};
use rust_decimal::Decimal;

use crate::dsl::schema_it;
use crate::errors;
use crate::for_impl::*;
use crate::schema::Schema;
use crate::stdlib::io::File;
use crate::sum_type::Case;
use crate::types::{DataType, NativeKind, Rel, RelShape, Tuple};
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
    #[display(fmt = "t'{}'", _0)]
    Time(Time),
    #[display(fmt = "d'{}'", _0)]
    Date(Date),
    #[display(fmt = "dt'{}'", _0)]
    DateTime(DateTime),
    //Strings
    #[display(fmt = "'{}'", _0)]
    Char(char),
    #[display(fmt = "'{}'", _0)]
    UTF8(Rc<String>),
    //Sum types
    Sum(Box<Case>),
    //Collections
    Vector(Rc<Vector>),
    //Lazy computation
    //Seq(Seq<'static>),
    File(File),
    Top,
}

impl Scalar {
    pub fn repeat(&self, times: usize) -> Tuple {
        (0..times).map(|_| self.clone()).collect()
    }

    pub fn rows_iter(&self) -> Box<dyn Iterator<Item = Tuple> + '_> {
        match self {
            Scalar::Vector(x) => Box::new(x.rows_iter()),
            Scalar::File(x) => Box::new(x.rows_iter()),
            x => Box::new(std::iter::once(x.clone()).map(|x| vec![x])),
        }
    }

    pub fn to_scalar(&self) -> Option<Scalar> {
        if !self.is_scalar() {
            return None;
        }

        match self {
            Scalar::Vector(x) => Some(x.data[0].clone()),
            Scalar::File(_) => None,
            x => Some(x.clone()),
        }
    }
}

impl Rel for Scalar {
    fn type_name(&self) -> &str {
        match self {
            Scalar::None => "None",
            Scalar::Bit(_) => "Bit",
            Scalar::Bool(_) => "Bool",
            Scalar::Char(_) => "Char",
            Scalar::Date(_) => "Date",
            Scalar::DateTime(_) => "DateTime",
            Scalar::Decimal(_) => "Decimal",
            Scalar::F64(_) => "F64",
            Scalar::I64(_) => "I64",
            Scalar::Time(_) => "Time",
            Scalar::UTF8(_) => "UTF8",
            Scalar::Sum(_) => "Sum",
            Scalar::Vector(x) => x.type_name(),
            Scalar::File(x) => x.type_name(),
            Scalar::Top => "Top",
        }
    }

    fn kind(&self) -> DataType {
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
            Scalar::File(x) => x.kind(),
            Scalar::Top => DataType::Top,
        }
    }

    fn schema(&self) -> Schema {
        match self {
            Scalar::Vector(x) => x.schema(),
            Scalar::File(x) => x.schema(),
            x => schema_it(x.kind()),
        }
    }

    fn len(&self) -> usize {
        match self {
            Scalar::Vector(x) => x.len(),
            Scalar::File(x) => x.len(),
            _ => 1,
        }
    }

    fn cols(&self) -> usize {
        match self {
            Scalar::Vector(x) => x.cols(),
            Scalar::File(x) => x.cols(),
            _ => 1,
        }
    }

    fn rows(&self) -> Option<usize> {
        match self {
            Scalar::Vector(x) => x.rows(),
            Scalar::File(x) => x.rows(),
            _ => Some(1),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        match self {
            Scalar::Vector(x) => x.rel_shape(),
            Scalar::File(x) => x.rel_shape(),
            _ => RelShape::Scalar,
        }
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        match self {
            Scalar::Vector(x) => x.rel_hash(&mut hasher),
            Scalar::File(x) => x.rel_hash(&mut hasher),
            x => x.hash(&mut hasher),
        }
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }
}

pub fn select(of: &[Scalar], cols: &[usize]) -> Tuple {
    if cols.is_empty() {
        vec![]
    } else {
        let mut cells = Vec::with_capacity(cols.len());
        for p in cols {
            cells.push(of[*p].clone());
        }
        cells
    }
}

pub(crate) fn combine(lhs: &[Scalar], rhs: &[Scalar]) -> Tuple {
    lhs.iter().chain(rhs.iter()).cloned().collect()
}

macro_rules! kind_native {
    ($native:ident, $kind:ident) => {
        impl NativeKind for $native {
            fn kind() -> DataType {
                DataType::$kind
            }
        }
    };
}

impl NativeKind for &str {
    fn kind() -> DataType {
        DataType::UTF8
    }
}

kind_native!(i64, I64);
kind_native!(bool, Bool);
kind_native!(Decimal, Decimal);
kind_native!(R64, F64);
kind_native!(f64, F64);
kind_native!(String, UTF8);

impl From<i32> for Scalar {
    fn from(x: i32) -> Self {
        Scalar::I64(x as i64)
    }
}

impl From<f64> for Scalar {
    fn from(x: f64) -> Self {
        Scalar::F64(x.into())
    }
}

impl From<&str> for Scalar {
    fn from(x: &str) -> Self {
        Scalar::UTF8(Rc::new(x.into()))
    }
}

impl From<String> for Scalar {
    fn from(x: String) -> Self {
        Scalar::UTF8(Rc::new(x))
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
        Scalar::Vector(Rc::new(x))
    }
}

macro_rules! convert {
    ($kind:ident, $bound:path) => {
        impl From<Scalar> for $kind {
            fn from(i: Scalar) -> Self {
                match i {
                    $bound(x) => x,
                    _ => unreachable!("{:?}", i),
                }
            }
        }

        impl<'a> From<&'a Scalar> for $kind {
            fn from(i: &'a Scalar) -> Self {
                match i {
                    $bound(x) => x.clone(),
                    _ => unreachable!("{:?}", i),
                }
            }
        }
    };
}

convert!(bool, Scalar::Bool);
convert!(i64, Scalar::I64);
convert!(R64, Scalar::F64);
convert!(Decimal, Scalar::Decimal);

impl From<Scalar> for String {
    fn from(i: Scalar) -> Self {
        match i {
            Scalar::UTF8(x) => x.to_string(),
            _ => unreachable!("{:?}", i),
        }
    }
}

/// Provide support for broadcast a function over scalars and vectors
pub fn fold_fn2<F>(x: &Scalar, y: &Scalar, apply: F) -> errors::Result<Scalar>
where
    F: Fn(&[Scalar]) -> errors::Result<Scalar>,
{
    let data = match (x, y) {
        (Scalar::Vector(a), Scalar::Vector(b)) => {
            if a.shape != b.shape {
                return Err(errors::Error::RankNotMatch);
            }
            let mut data = Vec::with_capacity(a.data.len());

            for (lhs, rhs) in a.data.iter().zip(b.data.iter()) {
                data.push(apply(&[lhs.clone(), rhs.clone()])?);
            }

            Ok(Vector::new_vector(data, a.kind()))
        }
        (_, Scalar::Vector(data)) => data.fold_fn(x, apply),
        (Scalar::Vector(data), _) => data.fold_fn(y, apply),
        _ => return Err(errors::Error::TypeMismatchBinOp(x.kind(), y.kind())),
    }?;
    Ok(data.into())
}
