use core::convert::Into;

use crate::for_impl::*;
use crate::prelude::*;
use crate::refcount::RefCount;

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

    pub fn is_scalar(&self) -> bool {
        self == &Self::scalar()
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
        let mut shape = self.clone();
        shape.start = col;
        shape.cols = 1;
        shape.stride = self.rows;
        shape
    }

    pub fn row(&self, row: usize) -> Shape {
        let mut shape = self.clone();
        shape.start = row;
        shape.end = row;
        shape
    }

    pub fn len(&self) -> usize {
        (self.cols * self.rows) / self.stride
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub shape: Shape,
    pub schema: Schema,
    pub data: RefCount<Vec<Scalar>>,
}

impl Vector {
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

    pub fn from_iter<'a, T: 'a>(xs: impl Iterator<Item = &'a [T]>, schema: Schema) -> Self
    where
        T: Into<Scalar> + Clone,
    {
        let mut iter = xs.peekable();
        let cols = iter.peek().map(|x| x.len()).unwrap_or(0);
        let data: Vec<Scalar> = iter
            .flat_map(|x| x.iter().cloned().map(Into::into))
            .collect();
        let shape = Shape::table(data.len() / cols, cols);

        Vector {
            data: RefCount::new(data),
            schema,
            shape,
        }
    }

    pub fn is_scalar(&self) -> bool {
        self.shape.is_scalar()
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

    pub fn rows_iter(&self) -> RowIter<'_> {
        RowIter {
            data: self,
            pos: 0,
            rows: self._rows(),
        }
    }

    pub fn col_iter(&self, col: usize) -> ColIter<'_> {
        ColIter {
            data: self,
            pos: 0,
            col,
            rows: self._rows(),
        }
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

    fn tuple(&self, pos: usize) -> Scalar {
        self.data[pos].clone()
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

pub struct RowIter<'a> {
    data: &'a Vector,
    pos: usize,
    rows: usize,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a [Scalar];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.rows {
            self.pos += 1;
            Some(self.data.row(self.pos - 1))
        } else {
            None
        }
    }
}

pub struct ColIter<'a> {
    data: &'a Vector,
    pos: usize,
    col: usize,
    rows: usize,
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Scalar;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.rows {
            self.pos += 1;
            Some(self.data.value(self.pos - 1, self.col))
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
            for (pos, x) in row.iter().enumerate() {
                write!(f, " {}", x)?;
                if pos < row.len() - 1 {
                    write!(f, ",")?;
                }
            }
            if row_pos < total - 1 {
                write!(f, ";")?;
            }
        }
        write!(f, "]")
    }
}
