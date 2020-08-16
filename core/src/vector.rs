use core::convert::Into;

use crate::for_impl::*;
use crate::prelude::*;
use crate::refcount::RefCount;
use crate::row::fmt_row;

/// Calculate the appropriated index in the flat array
#[inline]
pub fn index(col_count: usize, row: usize, col: usize) -> usize {
    //  _row_count: usize,
    // println!(
    //     "pos Row:{}, Col:{}, R:{}, C:{} = {}",
    //     row,
    //     col,
    //     row_count,
    //     col_count,
    //     row * col_count + col
    // );
    //    Layout::Col => col * row_count + row,
    //    Layout::Row => row * col_count + col,

    row * col_count + col
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shape {
    pub cols: usize,
    pub rows: usize,
    pub stride: usize,
    pub start: usize,
    pub end: usize,
}

impl Shape {
    pub fn scalar() -> Self {
        Shape {
            cols: 1,
            rows: 1,
            stride: 1,
            start: 0,
            end: 1,
        }
    }

    pub fn vector(rows: usize) -> Self {
        Shape {
            cols: 1,
            rows,
            stride: 1,
            start: 0,
            end: rows,
        }
    }

    pub fn table(rows: usize, cols: usize) -> Self {
        Shape {
            cols,
            rows,
            stride: rows,
            start: 0,
            end: rows * cols,
        }
    }

    pub fn column(&self, col: usize) -> Shape {
        let mut shape = *self;
        shape.start = col;
        shape.cols = 1;
        shape.stride = self.rows;
        shape
    }

    pub fn row(&self, row: usize) -> Shape {
        let mut shape = *self;
        shape.start = row;
        shape.end = row;
        shape
    }

    pub fn len(&self) -> usize {
        (self.cols * self.rows) / self.stride
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub shape: Shape,
    pub schema: Schema,
    pub data: RefCount<Vec<Scalar>>,
}

impl Vector {
    pub fn new_empty(kind: DataType) -> Self {
        Self::new_vector(vec![], kind)
    }

    pub fn new_scalar(data: Scalar) -> Self {
        let shape = Shape::scalar();
        let schema = schema_it(data.kind());
        Vector {
            data: RefCount::new(vec![data]),
            schema,
            shape,
        }
    }

    pub fn new_vector(data: Vec<Scalar>, kind: DataType) -> Self {
        let shape = Shape::vector(data.len());
        Vector {
            data: RefCount::new(data),
            schema: schema_it(kind),
            shape,
        }
    }

    pub fn new_table(data: Vec<Scalar>, schema: Schema) -> Self {
        let shape = Shape::table(data.len(), schema.len());
        Vector {
            data: RefCount::new(data),
            schema,
            shape,
        }
    }

    pub fn from_slice<T>(data: &[T], schema: Schema) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let shape = Shape::table(data.len(), 1);
        Vector {
            data: RefCount::new(data.iter().cloned().map(Into::into).collect()),
            schema,
            shape,
        }
    }

    pub fn from_iter<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = &'a [T]>) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let mut iter = xs.peekable();
        let cols = iter.peek().map(|x| x.len()).unwrap_or(0);
        let data: Vec<Scalar> = iter.flat_map(to_vec).collect();
        let shape = Shape::table(data.len() / cols, cols);

        Vector {
            data: RefCount::new(data),
            schema,
            shape,
        }
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        index(self.shape.cols, row, col)
    }

    pub fn value(&self, row: usize, col: usize) -> &Scalar {
        &self.data[self.index(row, col)]
    }

    pub fn col(&self, pos: usize) -> Vector {
        let mut col = self.clone();
        col.shape = col.shape.column(pos);
        col
    }

    fn _rows(&self) -> usize {
        self.data.len() / self.shape.cols
    }

    pub fn row(&self, row: usize) -> &[Scalar] {
        let start = self.index(row, 0);
        let end = start + self.shape.cols;
        &self.data[start..end]
    }

    pub fn rows_iter(&self) -> VectorIter<'_> {
        VectorIter::new_rows(self)
    }

    pub fn col_iter(&self, col: usize) -> VectorIter<'_> {
        VectorIter::new_col(self, col)
    }
}

impl Rel for Vector {
    fn type_name(&self) -> &str {
        "Vector"
    }

    fn kind(&self) -> DataType {
        let kinds = self.schema.kind();
        if kinds.len() == 1 {
            kinds[0].clone()
        } else {
            DataType::Map(kinds.into())
        }
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.shape.cols
    }

    fn rows(&self) -> Option<usize> {
        Some(self._rows())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        if self.shape.cols == 1 {
            if self.shape.rows == 1 {
                RelShape::Scalar
            } else {
                RelShape::Vec
            }
        } else {
            RelShape::Table
        }
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

pub struct VectorIter<'a> {
    data: &'a Vector,
    pos: usize,
    stride: usize,
    len: usize,
    end: usize,
}

impl<'a> VectorIter<'a> {
    pub fn new(data: &'a Vector, stride: usize, len: usize, start: usize, end: usize) -> Self {
        VectorIter {
            data,
            pos: start,
            len,
            stride,
            end,
        }
    }

    pub fn new_rows(data: &'a Vector) -> Self {
        Self::new(data, 0, data.cols(), 0, data.data.len())
    }

    pub fn new_col(data: &'a Vector, col: usize) -> Self {
        let rows = data._rows();
        Self::new(data, rows - 1, 1, col, data.data.len())
    }
}

//TODO: Implement the rest of methods, and support exact sized iterators...
impl<'a> Iterator for VectorIter<'a> {
    type Item = &'a [Scalar];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            let start = self.pos;
            let end = start + self.len;
            self.pos = end + self.stride;

            Some(&self.data.data[start..end])
        } else {
            None
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec[{};", self.kind())?;
        let total = self._rows();
        for (row_pos, row) in self.rows_iter().enumerate() {
            fmt_row(&row, f)?;
            if row_pos < total - 1 {
                write!(f, ";")?;
            }
        }
        write!(f, "]")
    }
}
