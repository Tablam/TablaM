use std::fmt;

use crate::refcount::RefCount;
use crate::scalar::Scalar;
use crate::types::DataType;

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
        unimplemented!()
    }

    pub fn row(&self, row: usize) -> Shape {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        (self.cols * self.rows) / self.stride
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub shape: Shape,
    pub kind: DataType,
    pub data: RefCount<Vec<Scalar>>,
}

impl Vector {
    pub fn new(data: Vec<Scalar>, kind: DataType, shape: Shape) -> Self {
        Vector {
            data: RefCount::new(data),
            kind,
            shape,
        }
    }

    pub fn new_scalar(data: Scalar) -> Self {
        let kind = data.kind();
        Self::new(vec![data], kind, Shape::scalar())
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        index(self.shape.cols, row, col)
    }

    pub fn len(&self) -> usize {
        self.shape.len()
    }

    pub fn rows(&self) -> usize {
        self.shape.rows
    }

    pub fn value(&self, row: usize, col: usize) -> &Scalar {
        &self.data[self.index(row, col)]
    }

    pub fn col(&self, pos: usize) -> Vector {
        let mut col = self.clone();
        col.shape = col.shape.column(pos);
        col
    }

    pub fn row(&self, row: usize) -> &[Scalar] {
        let start = self.index(row, 0);
        let end = start + self.shape.cols;
        &self.data[start..end]
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec[{};", self.kind)?;
        for x in self.data.as_slice() {
            write!(f, "{:?},", x)?;
        }
        write!(f, "]")
    }
}
