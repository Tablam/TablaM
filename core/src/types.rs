use std::any::Any;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use derive_more::Display;

use crate::scalar::Scalar;
use crate::schema::Schema;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindRel(Vec<DataType>);

impl fmt::Display for KindRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[|")?;
        for x in &self.0 {
            write!(f, "{}", x)?;
        }
        write!(f, "|]")
    }
}

impl From<Vec<DataType>> for KindRel {
    fn from(x: Vec<DataType>) -> Self {
        KindRel(x)
    }
}

//NOTE: This define a total order, so it matter what is the order of the enum!
//The overall sorting order is defined as:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum DataType {
    None,
    Bit,
    Bool,
    // Numeric
    I64,
    F64,
    Decimal,
    // Dates
    Time,
    Date,
    DateTime,
    // Text
    Char,
    UTF8,
    // For list, dynamic
    ANY,
    // Complex
    #[display(fmt = "Enum({})", _0)]
    Sum(Box<DataType>),
    #[display(fmt = "Vec({})", _0)]
    Vec(Box<DataType>),
    #[display(fmt = "Tree({})", _0)]
    Tree(KindRel),
    #[display(fmt = "Map({})", _0)]
    Map(KindRel),
    // Planed: Blob
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BinOp {
    Add,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum LogicOp {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CompareOp {
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
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

pub enum KeyValue {
    Key,
    Value,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ColumnAlias {
    pub from: Column,
    pub to: String,
}

impl ColumnAlias {
    pub fn new(from: Column, to: String) -> Self {
        ColumnAlias { from, to }
    }

    pub fn rename_pos(from: usize, to: &str) -> Self {
        Self::new(Column::Pos(from), to.into())
    }

    pub fn rename_name(from: &str, to: &str) -> Self {
        Self::new(Column::Name(from.into()), to.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Column {
    Pos(usize),
    Name(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelShape {
    Scalar,
    Vec,
    Table,
}

pub trait ToHash {
    fn to_hash(&self, h: &mut dyn Hasher);
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

pub fn is_t<T: 'static>(of: &dyn Rel) -> bool {
    std::any::TypeId::of::<T>() == of.as_any().type_id()
}

pub fn cmp_eq<T: 'static>(of: &T, other: &dyn Rel) -> bool
where
    T: PartialEq + Debug,
{
    //dbg!(&of.type_id(), &other.as_any().type_id());
    let y = other.as_any();

    if let Some(x) = y.downcast_ref::<T>() {
        of == x
    } else {
        false
    }
}

pub fn cmp<T: 'static>(of: &T, other: &dyn Rel) -> Ordering
where
    T: Ord + Rel,
{
    if let Some(x) = other.as_any().downcast_ref::<T>() {
        of.cmp(&x)
    } else {
        of.schema().cmp(&other.schema())
    }
}

pub trait Rel: Debug {
    fn type_name(&self) -> &str;

    fn kind(&self) -> DataType;

    fn schema(&self) -> Schema;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn cols(&self) -> usize;
    fn rows(&self) -> Option<usize>;

    fn as_any(&self) -> &dyn Any;

    fn tuple(&self, pos: usize) -> Scalar;

    fn rel_shape(&self) -> RelShape;
    fn rel_hash(&self, hasher: &mut dyn Hasher);
    fn rel_eq(&self, other: &dyn Rel) -> bool;
    fn rel_cmp(&self, other: &dyn Rel) -> Ordering;

    //fn as_iter(&self) -> Seq<'_>;
}

impl PartialEq for dyn Rel {
    fn eq(&self, other: &Self) -> bool {
        self.rel_eq(other)
    }
}
impl Eq for dyn Rel {}

impl PartialOrd for dyn Rel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rel_cmp(other))
    }
}

impl Ord for dyn Rel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rel_cmp(other)
    }
}

impl ToHash for dyn Rel {
    fn to_hash(&self, h: &mut dyn Hasher) {
        self.rel_hash(h)
    }
}

//Type Alias...
pub type Pos = Vec<usize>;
pub type BoolExpr = dyn Fn(&dyn Rel) -> bool;
pub type MapExpr = dyn Fn(&dyn Rel) -> Box<dyn Rel>;
pub type Iter<'a> = dyn Iterator<Item = Scalar> + 'a;
