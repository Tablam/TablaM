use indexmap::set::IndexSet;

use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, Clone, Eq)]
pub struct Map {
    pub schema: Schema,
    pk: usize,
    data: IndexSet<RowPk>,
}

impl Map {
    pub fn new(schema: Schema, data: IndexSet<RowPk>) -> Self {
        let pk = check_pk(&schema);
        Map { schema, pk, data }
    }

    pub fn empty(schema: Schema) -> Self {
        let pk = check_pk(&schema);
        Map {
            schema,
            pk,
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

    pub fn rows_iter(&self) -> impl Iterator<Item = Tuple> + '_ {
        self.data.iter().map(|x| x.data.clone())
    }

    pub fn col_iter(&self, col: usize) -> impl Iterator<Item = Tuple> + '_ {
        self.data.iter().map(move |x| x.data[col..col + 1].to_vec())
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        self.pk
            .cmp(&other.pk)
            .then(self.data.len().cmp(&other.data.len()))
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
        self.pk.hash(&mut hasher);
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
        self.pk.hash(state);
        self.schema.hash(state);
        for row in &self.data {
            row.hash(state);
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cols() > 0 {
            write!(f, "Map[{};", self.schema)?;
            let total = self._rows();
            for (row_pos, row) in self.data.iter().enumerate() {
                write!(f, "{}", row)?;
                if row_pos < total - 1 {
                    write!(f, ";")?;
                }
            }
            write!(f, "]")
        } else {
            write!(f, "Map[]")
        }
    }
}
