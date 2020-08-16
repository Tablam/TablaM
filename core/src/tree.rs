use std::collections::BTreeSet;

use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tree {
    pub schema: Schema,
    pk: usize,
    data: BTreeSet<RowPk>,
}

impl Tree {
    pub fn new(schema: Schema, data: BTreeSet<RowPk>) -> Self {
        let pk = check_pk(&schema);
        Tree { schema, pk, data }
    }

    pub fn empty(schema: Schema) -> Self {
        let pk = check_pk(&schema);
        Tree {
            schema,
            pk,
            data: BTreeSet::new(),
        }
    }

    pub fn from_iter<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = &'a [T]>) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let pk = check_pk(&schema);
        let data: BTreeSet<_> = xs.map(|x| RowPk::new(pk, to_vec(x))).collect();

        Self::new(schema, data)
    }

    fn _rows(&self) -> usize {
        self.data.len()
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = &[Scalar]> {
        self.data.iter().map(|x| x.data.as_slice())
    }

    pub fn col_iter(&self, col: usize) -> impl Iterator<Item = &[Scalar]> {
        self.data.iter().map(move |x| &x.data[col..col + 1])
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
        self.data.len() * self.cols()
    }

    fn cols(&self) -> usize {
        self.schema.len()
    }

    fn rows(&self) -> Option<usize> {
        Some(self._rows())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        RelShape::Table
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
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree[{};", self.schema)?;
        let total = self._rows();
        for (row_pos, row) in self.data.iter().enumerate() {
            write!(f, "{}", row)?;
            if row_pos < total - 1 {
                write!(f, ";")?;
            }
        }
        write!(f, "]")
    }
}
