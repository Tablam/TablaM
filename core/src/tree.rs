use std::collections::BTreeSet;

use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tree {
    pub schema: Schema,
    data: BTreeSet<RowPk>,
}

impl Tree {
    pub fn new(schema: Schema, data: BTreeSet<RowPk>) -> Self {
        check_pk(&schema);
        Tree { schema, data }
    }

    pub fn empty(schema: Schema) -> Self {
        check_pk(&schema);
        Tree {
            schema,
            data: BTreeSet::new(),
        }
    }

    pub fn from_iter<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = Vec<T>>) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let pk = check_pk(&schema);
        let data: BTreeSet<_> = xs.map(|x| RowPk::new(pk, to_vec(&x))).collect();

        Self::new(schema, data)
    }

    fn _rows(&self) -> usize {
        self.data.len()
    }
}

impl Rel for Tree {
    fn type_name(&self) -> &str {
        "Tree"
    }

    fn kind(&self) -> DataType {
        DataType::Tree(self.schema.kind().into())
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        self.data.len() * self.schema.len()
    }

    fn size(&self) -> ShapeLen {
        ShapeLen::Table(self.schema.len(), self.data.len())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        self.data.hash(&mut hasher)
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }

    fn iter(&self) -> Box<IterScalar<'_>> {
        Box::new(self.data.iter().map(|x: &RowPk| x.data.iter()).flatten())
    }

    fn col(&self, pos: usize) -> Col<'_> {
        let iter = self.data.iter().map(move |x: &RowPk| &x.data[pos]);
        Col::new(pos, Box::new(iter))
    }

    fn rows(&self) -> Box<IterRows<'_>> {
        Box::new(self.data.iter().map(Row::Tuple))
    }

    fn from_query(of: QueryResult<'_>) -> Self
    where
        Self: Sized,
    {
        Tree::from_iter(of.schema, of.iter.map(|x| x.to_vec()))
    }

    fn from_joins(of: QueryResultOwned<'_>) -> Self
    where
        Self: Sized,
    {
        Tree::from_iter(of.schema, of.iter.map(|x| x.to_vec()))
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_table(self.type_name(), &self.schema, self.size(), self.rows(), f)
    }
}
