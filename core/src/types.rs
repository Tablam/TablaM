use std::any::Any;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;

use derive_more::{Display, From};
use dyn_clone::DynClone;

use crate::query::QueryOp;
use crate::scalar::Scalar;
use crate::schema::Schema;

pub fn format_list<I>(
    list: impl IntoIterator<Item = I>,
    total: usize,
    start: &str,
    end: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    I: fmt::Display,
{
    write!(f, "{}", start)?;

    for (pos, x) in list.into_iter().enumerate() {
        if pos < total - 1 {
            write!(f, "{}, ", x)?;
        } else {
            write!(f, "{}", x)?;
        }
    }

    write!(f, "{}", end)
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
    Bit,
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
    UTF8,
    // Complex
    #[display(fmt = "{}...", _0)]
    Variadic(Box<DataType>),
    #[display(fmt = "Enum({})", _0)]
    Sum(Box<DataType>),
    #[display(fmt = "{}", _0)]
    Vec(KindFlat),
    #[display(fmt = "{}", _0)]
    Tuple(KindFlat),
    #[display(fmt = "Tree({})", _0)]
    Tree(KindRel),
    #[display(fmt = "Map({})", _0)]
    Map(KindRel),
    #[display(fmt = "Seq({})", _0)]
    Seq(KindRel),
    // Planed: Blob
    // For list, dynamic
    #[display(fmt = "Any")]
    ANY, //The TOP type
}

impl DataType {
    pub fn kind_group(&self) -> KindGroup {
        match self {
            DataType::I64 => KindGroup::Numbers,
            DataType::F64 => KindGroup::Numbers,
            DataType::Decimal => KindGroup::Numbers,
            DataType::Time => KindGroup::Dates,
            DataType::Date => KindGroup::Dates,
            DataType::DateTime => KindGroup::Dates,
            DataType::Char => KindGroup::Strings,
            DataType::UTF8 => KindGroup::Strings,
            _ => KindGroup::Other,
        }
    }

    pub fn default_value(&self) -> Scalar {
        match self {
            DataType::Unit => Scalar::Unit,
            DataType::Bit => Scalar::Bit(0),
            DataType::Bool => Scalar::Bool(false),
            DataType::I64 => Scalar::I64(0),
            DataType::F64 => Scalar::F64(0.0.into()),
            DataType::Decimal => Scalar::Decimal(0.into()),
            DataType::Time => unimplemented!(),
            DataType::Date => unimplemented!(),
            DataType::DateTime => unimplemented!(),
            DataType::Char => Scalar::Char(char::default()),
            DataType::UTF8 => Scalar::UTF8(Rc::new("".into())),
            DataType::ANY => Scalar::Unit,
            DataType::Variadic(_) => unimplemented!(),
            DataType::Sum(_) => unimplemented!(),
            DataType::Vec(_) => unimplemented!(),
            DataType::Tree(_) => unimplemented!(),
            DataType::Map(_) => unimplemented!(),
            DataType::Seq(_) => unimplemented!(),
            DataType::Tuple(_) => unimplemented!(),
        }
    }
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<DataType, Self::Err> {
        match input {
            "Bool" => Ok(DataType::Bool),
            "Dec" => Ok(DataType::Decimal),
            "Int" => Ok(DataType::I64),
            "Float" => Ok(DataType::F64),
            "Date" => Ok(DataType::Date),
            "Time" => Ok(DataType::Time),
            "DateTime" => Ok(DataType::DateTime),
            "Str" => Ok(DataType::UTF8),
            "Char" => Ok(DataType::Char),
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
    #[display(fmt = "+")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelShape {
    Scalar,
    Vec,
    Table,
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
    T: PartialEq + fmt::Debug,
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

pub trait Rel: fmt::Debug + DynClone {
    fn type_name(&self) -> &str;

    fn kind(&self) -> DataType;

    fn schema(&self) -> Schema;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn is_scalar(&self) -> bool {
        self.cols() == 1 && self.rows() == Some(1)
    }

    fn cols(&self) -> usize;
    fn rows(&self) -> Option<usize>;

    fn as_any(&self) -> &dyn Any;

    fn rel_shape(&self) -> RelShape;
    fn rel_hash(&self, hasher: &mut dyn Hasher);
    fn rel_eq(&self, other: &dyn Rel) -> bool;
    fn rel_cmp(&self, other: &dyn Rel) -> Ordering;

    fn query(&self) -> QueryOp {
        QueryOp::new(self.schema())
    }
}

#[derive(Debug, From)]
pub struct Relation {
    pub(crate) rel: Box<dyn Rel>,
}

impl Clone for Relation {
    fn clone(&self) -> Self {
        Relation {
            rel: dyn_clone::clone_box(&*self.rel),
        }
    }
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.rel.rel_eq(&*other.rel)
    }
}
impl Eq for Relation {}

impl PartialOrd for Relation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rel.rel_cmp(&*other.rel))
    }
}

impl Ord for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rel.rel_cmp(&*other.rel)
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rel.type_name())?;
        write!(f, "{}", self.rel.schema())?;
        Ok(())
    }
}

pub trait ToHash {
    fn to_hash(&self, h: &mut dyn Hasher);
}

impl ToHash for dyn Rel {
    fn to_hash(&self, h: &mut dyn Hasher) {
        self.rel_hash(h)
    }
}

impl Hash for Relation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rel.rel_hash(state);
    }
}

//Type Alias...
pub type Pos = Vec<usize>;
pub type Tuple = Vec<Scalar>;
pub type BoolExpr = dyn Fn(&dyn Rel) -> bool;
pub type MapExpr = dyn Fn(&dyn Rel) -> Box<dyn Rel>;
pub type Iter<'a> = dyn Iterator<Item = Tuple> + 'a;
pub type Iter2 = dyn Iterator<Item = Vec<Scalar>>;
