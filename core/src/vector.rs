use std::fmt::Debug;

use crate::for_impl::*;
use crate::prelude::*;

use crate::ndarray::ArrayView;
use derive_more::From;
use ndarray::{ArrayD, Axis, IxDyn, ShapeError};

fn _make_vector<T>(rows: usize, cols: usize, data: Vec<T>) -> Result<ArrayD<T>, ShapeError> {
    ArrayD::from_shape_vec(IxDyn(&[rows, cols]), data)
}

fn _make_scalar<T>(data: T) -> Result<ArrayD<T>, ShapeError> {
    ArrayD::from_shape_vec(IxDyn(&[1]), vec![data])
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub struct Vector {
    pub schema: Schema,
    pub data: ArrayD<Scalar>,
}

impl Vector {
    pub fn new(schema: Schema, data: ArrayD<Scalar>) -> Self {
        Vector { schema, data }
    }

    pub fn new_empty(kind: DataType) -> Self {
        Self::new_vector(vec![], kind)
    }

    pub fn new_scalar(data: Scalar) -> Self {
        let schema = schema_it(data.kind());
        Vector {
            data: _make_scalar(data).unwrap(),
            schema,
        }
    }

    pub fn new_vector(data: Vec<Scalar>, kind: DataType) -> Self {
        Vector {
            data: _make_vector(1, data.len(), data).unwrap(),
            schema: schema_it(kind),
        }
    }

    pub fn new_table(data: Vec<Scalar>, schema: Schema) -> Self {
        let cols = schema.len();
        let rows = if cols > 0 && data.len() > 0 {
            data.len() / cols
        } else {
            0
        };
        Vector {
            data: _make_vector(rows, cols, data).unwrap(),
            schema,
        }
    }

    pub fn from_iter<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = &'a T>) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let data = xs.cloned().map(Into::into).collect();
        Self::new_table(data, schema)
    }

    pub fn row(&self, row: usize) -> ArrayView<'_, Scalar, IxDyn> {
        self.data.index_axis(Axis(0), row)
    }

    pub fn col(&self, col: usize) -> Box<IterScalar<'_>> {
        let axis = if self.size().cols() == 1 { 0 } else { 1 };

        Box::new(self.data.index_axis(Axis(axis), col).into_iter())
    }
}

impl Rel for Vector {
    fn type_name(&self) -> &str {
        "Vec"
    }

    fn kind(&self) -> DataType {
        match self.size() {
            ShapeLen::Scalar => self.schema.fields[0].kind.clone(),
            ShapeLen::Vec(_) => DataType::Vec(Box::new(self.schema.fields[0].kind.clone())),
            ShapeLen::Table(_, _) => DataType::Vec2d(self.schema.kind().into()),
            ShapeLen::Iter(_, _) => unreachable!(),
        }
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> ShapeLen {
        let cols = self.schema.len();
        let rows = *self.data.shape().first().unwrap_or(&0);
        match (cols, rows) {
            (0, _) => ShapeLen::Vec(0),
            (1, 1) => ShapeLen::Scalar,
            (1, y) => ShapeLen::Vec(y),
            (x, y) => ShapeLen::Table(x, y),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher)
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }

    fn iter(&self) -> Box<IterScalar<'_>> {
        Box::new(self.data.iter())
    }

    fn cols(&self) -> Box<IterCols<'_>> {
        unimplemented!()
    }

    fn rows(&self) -> Box<IterRows<'_>> {
        Box::new(self.data.rows().into_iter().map(Row::Vector))
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size()
            .cmp(&other.size())
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

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_table(self.type_name(), &self.schema, self.size(), self.rows(), f)
    }
}
