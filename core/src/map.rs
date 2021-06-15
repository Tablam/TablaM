use indexmap::set::IndexSet;

use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, Clone, Eq)]
pub struct Map {
    pub schema: Schema,
    data: IndexSet<RowPk>,
}

impl Map {
    pub fn new(schema: Schema, data: IndexSet<RowPk>) -> Self {
        check_pk(&schema);
        Map { schema, data }
    }

    pub fn empty(schema: Schema) -> Self {
        check_pk(&schema);
        Map {
            schema,
            data: IndexSet::new(),
        }
    }

    pub fn from_iter<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = Vec<T>>) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let pk = check_pk(&schema);
        let data: IndexSet<_> = xs.map(|x| RowPk::new(pk, to_vec(&x))).collect();

        Self::new(schema, data)
    }

    fn _rows(&self) -> usize {
        self.data.len()
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        self.data
            .len()
            .cmp(&other.data.len())
            .then(self.schema.cmp(&other.schema))
            .then_with(|| {
                for (a, b) in self.data.iter().zip(other.data.iter()) {
                    let result = a.cmp(b);
                    if result != Ordering::Equal {
                        return result;
                    };
                }
                Ordering::Equal
            })
    }
}

impl Rel for Map {
    fn type_name(&self) -> &str {
        "Map"
    }

    fn kind(&self) -> DataType {
        DataType::Map(self.schema.kind().into())
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
        self.schema.hash(&mut hasher);
        for row in &self.data {
            row.hash(&mut hasher);
        }
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

    fn from_query(_of: QueryResult<'_>) -> Self
    where
        Self: Sized,
    {
        unreachable!()
    }

    fn from_joins(_of: QueryResultOwned<'_>) -> Self
    where
        Self: Sized,
    {
        unreachable!()
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.schema.hash(state);
        for row in &self.data {
            row.hash(state);
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_table(self.type_name(), &self.schema, self.size(), self.rows(), f)
    }
}
