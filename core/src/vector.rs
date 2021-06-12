use std::fmt::Debug;

use crate::for_impl::*;
use crate::prelude::*;

use derive_more::From;
use ndarray::{ArrayD, IxDyn, ShapeError};

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
        Vector {
            data: _make_vector(data.len() / cols, data.len(), data).unwrap(),
            schema,
        }
    }
}

impl Rel for Vector {
    fn type_name(&self) -> &str {
        "Vector"
    }

    fn kind(&self) -> DataType {
        DataType::Vec2d(self.schema.kind().into())
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> ShapeLen {
        let cols = self.schema.len();
        if cols > 0 {
            let rows = self.data.shape();
            if rows.is_empty() {
                ShapeLen::Vec(cols)
            } else {
                ShapeLen::Table(cols, rows[0])
            }
        } else {
            ShapeLen::Vec(0)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        RelShape::Vector
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
        let size = self.size();
        if size.cols() > 0 {
            write!(f, "Vec[{};", self.schema)?;
            let total = size.rows().unwrap_or_default();
            for (row_pos, row) in self.rows().enumerate() {
                if row_pos < total - 1 {
                    write!(f, "{} ;", row)?;
                } else {
                    write!(f, "{}", row)?;
                }
            }
            write!(f, "]")
        } else {
            write!(f, "Vec[]")
        }
    }
}
