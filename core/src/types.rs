use std::str::FromStr;

use derive_more::{Display, From};

use crate::for_impl::*;
use crate::function::Function;
use crate::prelude::Vector;
use crate::relation::{Rel, ToHash};
use crate::row::{Col, Row};
use crate::scalar::{Date, DateTime, Scalar, Time};
use crate::schema::{Field, Schema};
use crate::utils::format_list;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelShape {
    Scalar,
    Vector,
    Table,
    Iter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShapeLen {
    Scalar,
    Vec(usize),
    Table(usize, usize),
    Iter(usize, Option<usize>),
}

impl ShapeLen {
    pub fn rows(&self) -> Option<usize> {
        match self {
            ShapeLen::Scalar => Some(1),
            ShapeLen::Vec(x) => Some(*x),
            ShapeLen::Table(_, x) => Some(*x),
            ShapeLen::Iter(_, x) => *x,
        }
    }

    pub fn cols(&self) -> usize {
        match self {
            ShapeLen::Scalar => 1,
            ShapeLen::Vec(_) => 1,
            ShapeLen::Table(x, _) => *x,
            ShapeLen::Iter(x, _) => *x,
        }
    }

    pub fn is_scalar(&self) -> bool {
        self.cols() == 1 && self.rows() == Some(1)
    }

    fn is_bounded(&self) -> bool {
        self.rows().is_some()
    }
}

impl From<ShapeLen> for RelShape {
    fn from(of: ShapeLen) -> Self {
        match of {
            ShapeLen::Scalar => RelShape::Scalar,
            ShapeLen::Vec(cols) => {
                if cols == 1 {
                    RelShape::Scalar
                } else {
                    RelShape::Vector
                }
            }
            ShapeLen::Table(_, _) => {
                if of.is_scalar() {
                    RelShape::Scalar
                } else {
                    RelShape::Table
                }
            }
            ShapeLen::Iter(_, _) => RelShape::Iter,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindFlat(Vec<DataType>);

impl fmt::Display for KindFlat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(&self.0, self.0.len(), "", "", f)
    }
}

impl From<Vec<DataType>> for KindFlat {
    fn from(x: Vec<DataType>) -> Self {
        KindFlat(x)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindRel(Vec<DataType>);

impl fmt::Display for KindRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(&self.0, self.0.len(), "[|", "|]", f)
    }
}

impl From<Vec<DataType>> for KindRel {
    fn from(x: Vec<DataType>) -> Self {
        KindRel(x)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindFun {
    params: Vec<Field>,
    result: Box<Field>,
}

impl From<&Function> for KindFun {
    fn from(x: &Function) -> Self {
        KindFun {
            params: x.head.fields.clone(),
            result: Box::new(x.head.result.clone()),
        }
    }
}

impl fmt::Display for KindFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(&self.params, self.params.len(), "(", ")", f)?;

        write!(f, "= {}", self.result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum KindGroup {
    Numbers,
    Strings,
    Dates,
    Other,
}

//NOTE: This define a total order, so it matter what is the order of the enum!
//The overall sorting order is defined as:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum DataType {
    Unit, //The BOTTOM type
    Bool,
    // Numeric
    #[display(fmt = "Int")]
    I64,
    #[display(fmt = "Float")]
    F64,
    #[display(fmt = "Dec")]
    Decimal,
    // Dates
    Time,
    Date,
    DateTime,
    // Text
    Char,
    #[display(fmt = "Str")]
    Utf8,
    // Complex
    #[display(fmt = "{}...", _0)]
    Variadic(Box<DataType>),
    #[display(fmt = "Enum({})", _0)]
    Sum(KindFlat),
    #[display(fmt = "{}", _0)]
    Tuple(KindFlat),
    #[display(fmt = "{}", _0)]
    Vec(Box<DataType>),
    #[display(fmt = "{}", _0)]
    Vec2d(KindFlat),
    #[display(fmt = "Tree({})", _0)]
    Tree(KindRel),
    #[display(fmt = "Map({})", _0)]
    Map(KindRel),
    #[display(fmt = "Seq({})", _0)]
    Seq(KindRel),
    #[display(fmt = "Fun({})", _0)]
    Fun(KindFun),
    // Planed: Blob
    // For list, dynamic
    #[display(fmt = "Any")]
    Any, //The TOP type
}

impl DataType {
    pub fn shape(&self) -> RelShape {
        match self {
            DataType::I64
            | DataType::F64
            | DataType::Decimal
            | DataType::Time
            | DataType::Date
            | DataType::DateTime
            | DataType::Char
            | DataType::Utf8 => RelShape::Scalar,
            DataType::Vec(_) => RelShape::Vector,
            DataType::Seq(_) => RelShape::Iter,
            _ => RelShape::Table,
        }
    }

    pub fn kind_group(&self) -> KindGroup {
        match self {
            DataType::I64 | DataType::F64 | DataType::Decimal => KindGroup::Numbers,
            DataType::Time | DataType::Date | DataType::DateTime => KindGroup::Dates,
            DataType::Char | DataType::Utf8 => KindGroup::Strings,
            _ => KindGroup::Other,
        }
    }

    pub fn default_value(&self) -> Scalar {
        match self {
            DataType::Unit => Scalar::Unit,
            DataType::Bool => Scalar::Bool(false),
            DataType::I64 => Scalar::I64(0),
            DataType::F64 => Scalar::F64(0.0.into()),
            DataType::Decimal => Scalar::Decimal(0.into()),
            DataType::Time => Scalar::Time(Time::from_hms(0, 0, 0)),
            DataType::Date => Scalar::Date(Date::from_utc(
                chrono::MIN_DATE.naive_utc(),
                chrono::FixedOffset::east(0),
            )),
            DataType::DateTime => Scalar::DateTime(DateTime::from_utc(
                chrono::MIN_DATETIME.naive_utc(),
                chrono::FixedOffset::east(0),
            )),
            DataType::Char => Scalar::Char(char::default()),
            DataType::Utf8 => Scalar::Utf8(Rc::new("".into())),
            DataType::Variadic(_) => unimplemented!(),
            DataType::Sum(_) => unimplemented!(),
            DataType::Vec(x) => Rc::new(Vector::new_empty(*x.clone())).into(),
            DataType::Vec2d(_) => unreachable!(),
            DataType::Tree(_) => unimplemented!(),
            DataType::Map(_) => unimplemented!(),
            DataType::Seq(_) => unimplemented!(),
            DataType::Tuple(_) => unimplemented!(),
            DataType::Fun(_) => unreachable!(),
            DataType::Any => Scalar::Unit,
        }
    }
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<DataType, Self::Err> {
        match input {
            "Unit" => Ok(DataType::Unit),
            "Bool" => Ok(DataType::Bool),
            "Int" => Ok(DataType::I64),
            "Float" => Ok(DataType::F64),
            "Dec" => Ok(DataType::Decimal),
            "Time" => Ok(DataType::Time),
            "Date" => Ok(DataType::Date),
            "DateTime" => Ok(DataType::DateTime),
            "Char" => Ok(DataType::Char),
            "Str" => Ok(DataType::Utf8),
            x => Err(x.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum UnaryOp {
    #[display(fmt = "not")]
    Not,
    #[display(fmt = "-")]
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum BinOp {
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "/")]
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum LogicOp {
    #[display(fmt = "and")]
    And,
    #[display(fmt = "or")]
    Or,
    #[display(fmt = "=")]
    Equal,
    #[display(fmt = "<>")]
    NotEqual,
    #[display(fmt = ">")]
    Greater,
    #[display(fmt = ">=")]
    GreaterEqual,
    #[display(fmt = "<")]
    Less,
    #[display(fmt = "<=")]
    LessEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum CmOp {
    #[display(fmt = "=")]
    Eq,
    #[display(fmt = "<>")]
    NotEq,
    #[display(fmt = "<")]
    Less,
    #[display(fmt = "<=")]
    LessEq,
    #[display(fmt = ">")]
    Greater,
    #[display(fmt = ">=")]
    GreaterEq,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display, From)]
pub enum Comparable {
    #[display(fmt = "#{}", _0)]
    Name(String),
    #[display(fmt = "#{}", _0)]
    Column(usize),
    #[display(fmt = "{}", _0)]
    Scalar(Scalar),
}

impl Comparable {
    fn get_value<'a>(&'a self, schema: &Schema, row: &'a [Scalar]) -> &'a Scalar {
        match self {
            Comparable::Column(pos) => &row[*pos],
            Comparable::Scalar(x) => x,
            Comparable::Name(name) => {
                let (pos, _) = schema.resolve_name(&Column::Name(name.into()));
                &row[pos]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CompareOp {
    pub op: CmOp,
    pub lhs: Comparable,
    pub rhs: Comparable,
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

macro_rules! cmp_fn {
    ($name:ident, $fun:ident) => {
        pub fn $name(schema: &Schema, row: &[Scalar], lhs: &Comparable, rhs: &Comparable) -> bool {
            let lhs = lhs.get_value(schema, row);
            let rhs = rhs.get_value(schema, row);
            lhs.$fun(rhs)
        }
    };
}
impl CompareOp {
    cmp_fn!(fn_eq, eq);
    cmp_fn!(fn_not_eq, ne);
    cmp_fn!(fn_less, lt);
    cmp_fn!(fn_less_eq, le);
    cmp_fn!(fn_greater, gt);
    cmp_fn!(fn_greater_eq, ge);

    pub fn get_fn(&self) -> &dyn Fn(&Schema, &[Scalar], &Comparable, &Comparable) -> bool {
        match self.op {
            CmOp::Eq => &Self::fn_eq,
            CmOp::NotEq => &Self::fn_not_eq,
            CmOp::Less => &Self::fn_less,
            CmOp::LessEq => &Self::fn_less_eq,
            CmOp::Greater => &Self::fn_greater,
            CmOp::GreaterEq => &Self::fn_greater_eq,
        }
    }
}

impl CompareOp {
    pub fn new(op: CmOp, lhs: Comparable, rhs: Comparable) -> Self {
        CompareOp { op, lhs, rhs }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CrudOp {
    Create,
    Update,
    Delete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IndexOp {
    Pos,
    Name,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SortDef {
    Asc(Column),
    Desc(Column),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ProjectDef {
    Select(Vec<Column>),
    Deselect(Vec<Column>),
}

impl ProjectDef {
    pub(crate) fn columns(&self) -> &[Column] {
        match self {
            ProjectDef::Select(x) => x,
            ProjectDef::Deselect(x) => x,
        }
    }
}

impl fmt::Display for ProjectDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = match self {
            ProjectDef::Select(cols) => {
                write!(f, "?select ")?;
                cols
            }
            ProjectDef::Deselect(cols) => {
                write!(f, "?deselect ")?;
                cols
            }
        };
        format_list(cols, cols.len(), "", "", f)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
#[display(fmt = "{} as #{}", from, to)]
pub struct ColumnAlias {
    pub from: Column,
    pub to: String,
}

impl ColumnAlias {
    pub fn new(from: Column, to: &str) -> Self {
        ColumnAlias {
            from,
            to: to.into(),
        }
    }

    pub fn rename_pos(from: usize, to: &str) -> Self {
        Self::new(Column::Pos(from), to)
    }

    pub fn rename_name(from: &str, to: &str) -> Self {
        Self::new(Column::Name(from.into()), to)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display, From)]
pub enum Column {
    #[display(fmt = "#{}", _0)]
    Pos(usize),
    #[display(fmt = "#{}", _0)]
    Name(String),
    #[display(fmt = "{}", _0)]
    Alias(Box<ColumnAlias>),
}

pub trait Value: Clone + PartialEq + PartialOrd + Eq + Ord + ToHash {}
impl<T: Clone + PartialEq + PartialOrd + Eq + Ord + ToHash> Value for T {}

pub trait NativeKind {
    fn kind() -> DataType;
}

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub fn type_of_t<T>() -> &'static str {
    std::any::type_name::<T>()
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", type_of_t::<T>())
}

pub fn as_t<T: 'static>(of: &dyn Rel) -> Option<&T> {
    if let Some(x) = of.as_any().downcast_ref::<&T>() {
        return Some(x);
    }
    None
}

pub fn as_t_cloned<T: Clone + 'static>(of: &dyn Rel) -> Option<T> {
    if let Some(x) = of.as_any().downcast_ref::<T>() {
        return Some(x.clone());
    }
    None
}

pub enum ScalarNative<'a, T> {
    One(&'a T),
    Slice(&'a [T]),
}
//Type Alias...
pub type Pos = Vec<usize>;
pub type Tuple = Vec<Scalar>;
pub type BoolExpr = dyn Fn(&dyn Rel) -> bool;
pub type MapExpr = dyn Fn(&dyn Rel) -> Box<dyn Rel>;
pub type IterScalar<'a> = dyn Iterator<Item = &'a Scalar> + 'a;
pub type IterRows<'a> = dyn Iterator<Item = Row<'a>> + 'a;
pub type IterCols<'a> = dyn Iterator<Item = Col<'a>> + 'a;
