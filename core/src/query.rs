use derive_more::Display;
use itertools::Itertools;

use crate::for_impl::*;
use crate::joins;
use crate::joins::Join;
use crate::prelude::*;
use crate::scalar::select;
use crate::types::ProjectDef;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum Comparable {
    #[display(fmt = "#{}", _0)]
    Column(usize),
    #[display(fmt = "{}", _0)]
    Scalar(Scalar),
}

impl Comparable {
    fn get_value<'a>(&'a self, row: &'a [Scalar]) -> &'a Scalar {
        match self {
            Comparable::Column(pos) => &row[*pos],
            Comparable::Scalar(x) => x,
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
        pub fn $name(row: &[Scalar], lhs: &Comparable, rhs: &Comparable) -> bool {
            let lhs = lhs.get_value(row);
            let rhs = rhs.get_value(row);
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

    pub fn get_fn(&self) -> &dyn Fn(&[Scalar], &Comparable, &Comparable) -> bool {
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub struct Project {
    cols: ProjectDef,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum Query {
    #[display(fmt = "?where {}", _0)]
    Filter(CompareOp),
    Project(Project),
    #[display(fmt = "?limit {}", _0)]
    Limit(usize),
    #[display(fmt = "?skip {}", _0)]
    Skip(usize),
    #[display(fmt = "?distinct")]
    Distinct,
}

pub struct QueryResult<'a> {
    pub schema: Schema,
    pub iter: Iter<'a>,
}

impl<'a> QueryResult<'a> {
    pub fn new(schema: Schema, iter: Iter<'a>) -> Self {
        QueryResult { schema, iter }
    }
}

pub struct QueryResultOwned<'a> {
    pub schema: Schema,
    pub iter: IterOwned<'a>,
}

impl<'a> QueryResultOwned<'a> {
    pub fn new(schema: Schema, iter: IterOwned<'a>) -> Self {
        QueryResultOwned { schema, iter }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct QueryOp {
    pub schema: Schema,
    query: Vec<Query>,
}

impl QueryOp {
    pub fn new(schema: Schema) -> Self {
        QueryOp {
            schema,
            query: Vec::new(),
        }
    }

    pub fn filter(mut self, op: CmOp, lhs: Comparable, rhs: Comparable) -> Self {
        let q = Query::Filter(CompareOp::new(op, lhs, rhs));
        self.query.push(q);
        self
    }

    pub fn eq(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::Eq, lhs, rhs)
    }

    pub fn not_eq(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::NotEq, lhs, rhs)
    }

    pub fn less(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::Less, lhs, rhs)
    }

    pub fn less_eq(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::LessEq, lhs, rhs)
    }

    pub fn greater(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::Greater, lhs, rhs)
    }

    pub fn greater_eq(self, lhs: Comparable, rhs: Comparable) -> Self {
        self.filter(CmOp::GreaterEq, lhs, rhs)
    }

    pub fn project(mut self, cols: ProjectDef) -> Self {
        let q = Query::Project(Project { cols });
        self.query.push(q);
        self
    }

    pub fn select(self, pos: &[Column]) -> Self {
        self.project(ProjectDef::Select(pos.to_vec()))
    }

    pub fn deselect(self, pos: &[Column]) -> Self {
        self.project(ProjectDef::Deselect(pos.to_vec()))
    }

    pub fn limit(mut self, rows: usize) -> Self {
        let q = Query::Limit(rows);
        self.query.push(q);
        self
    }

    pub fn skip(mut self, rows: usize) -> Self {
        let q = Query::Skip(rows);
        self.query.push(q);
        self
    }

    pub fn distinct(mut self) -> Self {
        let q = Query::Distinct;
        self.query.push(q);
        self
    }

    pub fn execute<'a>(self, iter: impl Iterator<Item = Tuple> + 'a) -> QueryResult<'a> {
        let mut result = Box::new(iter) as Iter<'a>;
        let mut schema = self.schema;
        for q in self.query {
            result = match q {
                Query::Filter(cmp) => {
                    let iter = result.filter(move |row| {
                        let apply = cmp.get_fn();
                        (apply)(row, &cmp.lhs, &cmp.rhs)
                    });
                    Box::new(iter)
                }
                Query::Project(columns) => {
                    let (new, cols) = schema.project(&columns.cols);
                    //dbg!(&schema, &new);
                    schema = new;
                    let iter = result.map(move |x| select(&x, &cols));
                    Box::new(iter)
                }
                Query::Limit(rows) => {
                    let iter = result.take(rows);
                    Box::new(iter)
                }
                Query::Skip(rows) => {
                    let iter = result.skip(rows);
                    Box::new(iter)
                }
                Query::Distinct => {
                    let iter = result.unique();
                    Box::new(iter)
                }
            }
        }
        QueryResult::new(schema, result)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum JoinOp {
    #[display(fmt = "{} {} {}", _0, _1, _2)]
    Join(Join, Schema, Schema),
    #[display(fmt = "union {} {}", _0, _1)]
    Union(Schema, Schema),
    #[display(fmt = "diff {} {}", _0, _1)]
    Diff(Schema, Schema),
    #[display(fmt = "union {} {}", _0, _1)]
    Intersect(Schema, Schema),
}

impl JoinOp {
    pub fn cross(lhs: Schema, rhs: Schema) -> Self {
        JoinOp::Join(Join::Cross, lhs, rhs)
    }
    pub fn join_left(lhs: Schema, rhs: Schema) -> Self {
        JoinOp::Join(Join::Left, lhs, rhs)
    }

    pub fn union(lhs: Schema, rhs: Schema) -> Result<Self> {
        if lhs == rhs {
            Ok(JoinOp::Union(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn diff(lhs: Schema, rhs: Schema) -> Result<Self> {
        if lhs == rhs {
            Ok(JoinOp::Diff(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn intersect(lhs: Schema, rhs: Schema) -> Result<Self> {
        if lhs == rhs {
            Ok(JoinOp::Intersect(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn execute<'a>(
        self,
        lhs: impl Iterator<Item = Tuple> + 'a,
        rhs: impl Iterator<Item = Tuple> + 'a,
    ) -> QueryResultOwned<'a> {
        match self {
            JoinOp::Join(join, ls, rs) => match join {
                Join::Cross => {
                    let schema = ls.extend(&rs);

                    let iter = joins::cross(lhs, rhs);
                    QueryResultOwned::new(schema, Box::new(iter))
                }
                Join::Left => {
                    let schema = ls.extend(&rs);

                    let iter = joins::left_join(lhs, rhs, rs.len());
                    QueryResultOwned::new(schema, Box::new(iter))
                }
                _ => unimplemented!(),
            },
            JoinOp::Union(ls, _) => {
                let iter = lhs.chain(rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
            JoinOp::Diff(ls, _) => {
                let iter = joins::difference(lhs, rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
            JoinOp::Intersect(ls, _) => {
                let iter = joins::intersect(lhs, rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
        }
    }
}

pub type Iter<'a> = Box<dyn Iterator<Item = Tuple> + 'a>;
pub type IterOwned<'a> = Box<dyn Iterator<Item = Vec<Scalar>> + 'a>;

pub type Chain<'a> = Box<dyn Fn(Iter<'a>) -> Iter<'a> + 'a>;
pub type Combinator<'a> = Box<dyn Fn(Iter, Iter) -> Iter<'a> + 'a>;

impl fmt::Display for QueryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for q in &self.query {
            write!(f, "{}", q)?;
        }
        Ok(())
    }
}
