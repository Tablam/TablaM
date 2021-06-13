use crate::for_impl::*;
use crate::prelude::*;

use crate::function::Function;
use derive_more::{Display, From, TryInto};

pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
pub type Date = chrono::Date<chrono::FixedOffset>;
pub type Time = chrono::NaiveTime;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, TryInto)]
pub enum Scalar {
    Unit,
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
    Utf8(Rc<String>),
    //Sum types
    Sum(Box<SumVariant>),
    //Collections
    Vector(Rc<Vector>),
    Tree(Rc<Tree>),
    Map(Rc<Map>),
    //Lazy computation
    Seq(Rc<Seq>),
    //Objects
    Fun(Box<Function>),
    Rel(RelationDyn),
    Top,
}

impl Scalar {
    pub fn repeat(&self, times: usize) -> Tuple {
        (0..times).map(|_| self.clone()).collect()
    }
}

impl Rel for Scalar {
    fn type_name(&self) -> &str {
        match self {
            Scalar::Unit => "None",
            Scalar::Bool(_) => "Bool",
            Scalar::Char(_) => "Char",
            Scalar::Date(_) => "Date",
            Scalar::DateTime(_) => "DateTime",
            Scalar::Decimal(_) => "Decimal",
            Scalar::F64(_) => "F64",
            Scalar::I64(_) => "I64",
            Scalar::Time(_) => "Time",
            Scalar::Utf8(_) => "Str",
            Scalar::Sum(_) => "Sum",
            Scalar::Vector(x) => x.type_name(),
            //Scalar::Tuple(x) => x.type_name(),
            Scalar::Tree(x) => x.type_name(),
            Scalar::Map(x) => x.type_name(),
            // Scalar::File(x) => x.type_name(),
            Scalar::Fun(x) => x.type_name(),
            Scalar::Rel(x) => x.rel.type_name(),
            Scalar::Seq(_) => "Seq",
            Scalar::Top => "Top",
        }
    }

    fn kind(&self) -> DataType {
        match self {
            Scalar::Unit => DataType::Unit,
            Scalar::Bool(_) => DataType::Bool,
            Scalar::Char(_) => DataType::Char,
            Scalar::Date(_) => DataType::Date,
            Scalar::DateTime(_) => DataType::DateTime,
            Scalar::Decimal(_) => DataType::Decimal,
            Scalar::F64(_) => DataType::F64,
            Scalar::I64(_) => DataType::I64,
            Scalar::Time(_) => DataType::Time,
            Scalar::Utf8(_) => DataType::Utf8,
            Scalar::Sum(x) => x.kind(),
            // Scalar::Tuple(x) => x.kind(),
            Scalar::Vector(x) => x.kind(),
            Scalar::Tree(x) => x.kind(),
            Scalar::Map(x) => x.kind(),
            Scalar::Rel(x) => x.rel.kind(),
            //Scalar::File(x) => x.kind(),
            Scalar::Fun(x) => x.kind(),
            Scalar::Seq(x) => DataType::Seq(x.schema.kind().into()),
            Scalar::Top => DataType::Any,
        }
    }

    fn schema(&self) -> Schema {
        match self {
            Scalar::Vector(x) => x.schema(),
            //Scalar::Tuple(x) => x.schema(),
            Scalar::Tree(x) => x.schema(),
            Scalar::Map(x) => x.schema(),
            Scalar::Rel(x) => x.rel.schema(),
            //Scalar::File(x) => x.schema(),
            Scalar::Fun(x) => x.schema(),
            x => schema_it(x.kind()),
        }
    }

    fn len(&self) -> usize {
        match self {
            Scalar::Vector(x) => x.len(),
            //Scalar::Tuple(x) => x.len(),
            Scalar::Tree(x) => x.len(),
            Scalar::Map(x) => x.len(),
            Scalar::Rel(x) => x.rel.len(),
            //Scalar::File(x) => x.len(),
            Scalar::Fun(x) => x.len(),
            _ => 1,
        }
    }

    fn size(&self) -> ShapeLen {
        match self {
            Scalar::Vector(x) => x.size(),
            //Scalar::Tuple(x) => x.size(),
            Scalar::Tree(x) => x.size(),
            Scalar::Map(x) => x.size(),
            Scalar::Rel(x) => x.rel.size(),
            //Scalar::File(x) => x.size(),
            Scalar::Fun(x) => x.size(),
            _ => ShapeLen::Scalar,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        match self {
            Scalar::Vector(x) => x.rel_hash(&mut hasher),
            //Scalar::Tuple(x) => x.rel_hash(&mut hasher),
            Scalar::Tree(x) => x.rel_hash(&mut hasher),
            Scalar::Map(x) => x.rel_hash(&mut hasher),
            Scalar::Rel(x) => x.rel.rel_hash(&mut hasher),
            //Scalar::File(x) => x.rel_hash(&mut hasher),
            Scalar::Fun(x) => x.rel_hash(&mut hasher),
            x => x.hash(&mut hasher),
        }
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }

    fn as_i64(&self) -> Option<ScalarNative<i64>> {
        match self {
            Scalar::I64(x) => Some(ScalarNative::One(x)),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<ScalarNative<Rc<String>>> {
        match self {
            Scalar::Utf8(x) => Some(ScalarNative::One(x)),
            _ => None,
        }
    }

    fn iter(&self) -> Box<IterScalar<'_>> {
        match self {
            Scalar::Vector(x) => x.iter(),
            //Scalar::Tuple(x) => x.rel_hash(&mut hasher),
            Scalar::Tree(x) => x.iter(),
            Scalar::Map(x) => x.iter(),
            Scalar::Rel(x) => x.rel.iter(),
            //Scalar::File(x) => x.iter(),
            Scalar::Fun(x) => x.iter(),
            x => Box::new(std::iter::once(x)),
        }
    }

    fn col(&self, pos: usize) -> Col<'_> {
        match self {
            Scalar::Vector(x) => x.col(pos),
            //Scalar::Tuple(x) => x.x.col(pos),
            Scalar::Tree(x) => x.col(pos),
            Scalar::Map(x) => x.col(pos),
            Scalar::Rel(x) => x.rel.col(pos),
            //Scalar::File(x) => x.col(pos),
            Scalar::Fun(x) => x.col(pos),
            x => Col::new(pos, Box::new(std::iter::once(x))),
        }
    }

    fn rows(&self) -> Box<IterRows<'_>> {
        match self {
            Scalar::Vector(x) => x.rows(),
            //Scalar::Tuple(x) => x.rel_hash(&mut hasher),
            Scalar::Tree(x) => x.rows(),
            Scalar::Map(x) => x.rows(),
            Scalar::Rel(x) => x.rel.rows(),
            //Scalar::File(x) => x.cols(),
            Scalar::Fun(x) => x.rows(),
            x => Box::new(std::iter::once(Row::Scalar(x))),
        }
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

pub fn combine(lhs: &[Scalar], rhs: &[Scalar]) -> Tuple {
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
        DataType::Utf8
    }
}

kind_native!(i64, I64);
kind_native!(bool, Bool);
kind_native!(Decimal, Decimal);
kind_native!(R64, F64);
kind_native!(f64, F64);
kind_native!(String, Utf8);

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
        Scalar::Utf8(Rc::new(x.into()))
    }
}

impl From<&char> for Scalar {
    fn from(x: &char) -> Self {
        Scalar::Char(*x)
    }
}

impl From<String> for Scalar {
    fn from(x: String) -> Self {
        Scalar::Utf8(Rc::new(x))
    }
}

impl From<&Rc<String>> for Scalar {
    fn from(x: &Rc<String>) -> Self {
        Scalar::Utf8(x.clone())
    }
}

impl From<Box<Scalar>> for Scalar {
    fn from(x: Box<Scalar>) -> Self {
        *x
    }
}

impl From<SumVariant> for Scalar {
    fn from(x: SumVariant) -> Self {
        Scalar::Sum(Box::new(x))
    }
}

impl From<Scalar> for String {
    fn from(i: Scalar) -> Self {
        match i {
            Scalar::Utf8(x) => x.to_string(),
            _ => unreachable!("{:?}", i),
        }
    }
}
