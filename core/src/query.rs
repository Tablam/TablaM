use crate::for_impl::*;
use crate::prelude::*;

use crate::iterators;
use crate::iterators::Join;
use crate::scalar::select;
use derive_more::{Display, From};
use itertools::Itertools;

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
    fn resolve_pos(&self, schema: &Schema) -> usize {
        match self {
            Comparable::Column(pos) => *pos,
            Comparable::Scalar(_) => unreachable!(),
            Comparable::Name(name) => {
                let (pos, _) = schema.resolve_name(&Column::Name(name.into()));
                pos
            }
        }
    }
    //
    // fn get_value(&self, schema: &Schema, row: &Row<'_>) -> &Scalar {
    //     match self {
    //         Comparable::Column(pos) => row.get(*pos).unwrap(),
    //         Comparable::Scalar(x) => x,
    //         Comparable::Name(name) => {
    //             let (pos, _) = schema.resolve_name(&Column::Name(name.into()));
    //             row.get(pos).unwrap()
    //         }
    //     }
    // }
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

impl CompareOp {
    pub fn cmp(&self, schema: &Schema, row: &Row<'_>) -> bool {
        let lhs = if let Comparable::Scalar(x) = &self.lhs {
            x
        } else {
            row.get(self.lhs.resolve_pos(schema)).unwrap()
        };

        let rhs = if let Comparable::Scalar(x) = &self.rhs {
            x
        } else {
            row.get(self.rhs.resolve_pos(schema)).unwrap()
        };

        match self.op {
            CmOp::Eq => lhs == rhs,
            CmOp::NotEq => lhs != rhs,
            CmOp::Less => lhs < rhs,
            CmOp::LessEq => lhs <= rhs,
            CmOp::Greater => lhs > rhs,
            CmOp::GreaterEq => lhs >= rhs,
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
    pub iter: Box<IterRows<'a>>,
}

impl<'a> QueryResult<'a> {
    pub fn new(schema: Schema, iter: Box<IterRows<'a>>) -> Self {
        QueryResult { schema, iter }
    }
}

pub struct QueryResultOwned<'a> {
    pub schema: Schema,
    pub iter: Box<IterRows<'a>>,
}

impl<'a> QueryResultOwned<'a> {
    pub fn new(schema: Schema, iter: Box<IterRows<'a>>) -> Self {
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

    pub fn execute<'a>(self, iter: impl Iterator<Item = Row<'a>> + 'a) -> QueryResult<'a> {
        let mut result = Box::new(iter) as Box<IterRows<'a>>;
        let mut schema = self.schema;
        for q in self.query {
            result = match q {
                Query::Filter(cmp) => {
                    let schema2 = schema.clone();
                    let iter = result.filter(move |row| cmp.cmp(&schema2, row));
                    Box::new(iter)
                }
                Query::Project(columns) => {
                    let (new, cols) = schema.project(&columns.cols);
                    //dbg!(&schema, &new);
                    schema = new;
                    let iter = result.map(move |x| select(&x, cols.clone()));
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

    pub fn union(lhs: Schema, rhs: Schema) -> ResultT<Self> {
        if lhs == rhs {
            Ok(JoinOp::Union(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn diff(lhs: Schema, rhs: Schema) -> ResultT<Self> {
        if lhs == rhs {
            Ok(JoinOp::Diff(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn intersect(lhs: Schema, rhs: Schema) -> ResultT<Self> {
        if lhs == rhs {
            Ok(JoinOp::Intersect(lhs, rhs))
        } else {
            Err(Error::SchemaNotMatchExact)
        }
    }

    pub fn execute<'a>(
        self,
        lhs: impl Iterator<Item = Row<'a>> + 'a,
        rhs: impl Iterator<Item = Row<'a>> + 'a,
    ) -> QueryResultOwned<'a> {
        match self {
            JoinOp::Join(join, ls, rs) => match join {
                Join::Cross => {
                    let schema = ls.extend(&rs);

                    let iter = iterators::cross(lhs, rhs);
                    QueryResultOwned::new(schema, Box::new(iter))
                }
                Join::Left => {
                    let schema = ls.extend(&rs);

                    let iter = iterators::left_join(lhs, rhs, rs.len());
                    QueryResultOwned::new(schema, Box::new(iter))
                }
                _ => unimplemented!(),
            },
            JoinOp::Union(ls, _) => {
                let iter = lhs.chain(rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
            JoinOp::Diff(ls, _) => {
                let iter = iterators::difference(lhs, rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
            JoinOp::Intersect(ls, _) => {
                let iter = iterators::intersect(lhs, rhs);
                QueryResultOwned::new(ls, Box::new(iter))
            }
        }
    }
}

pub type Chain<'a> = Box<dyn Fn(IterRows<'a>) -> IterRows<'a> + 'a>;
pub type Combinator<'a> = Box<dyn Fn(IterRows<'a>, IterRows<'a>) -> IterRows<'a> + 'a>;

impl fmt::Display for QueryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self.query.iter().map(|q| q.to_string()).join(" ");
        write!(f, "{}", joined)?;
        Ok(())
    }
}
