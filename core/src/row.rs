use crate::for_impl::*;
use crate::ndarray::ArrayView1;
use crate::prelude::*;

pub enum Col<'a> {
    Scalar(&'a Scalar),
}

pub enum Row<'a> {
    Scalar(&'a Scalar),
    Vector(ArrayView1<'a, Scalar>),
    Tuple(&'a RowPk),
}

impl<'a> Row<'a> {
    fn len(&self) -> usize {
        match self {
            Row::Scalar(x) => x.len(),
            Row::Vector(x) => x.len(),
            Row::Tuple(x) => x.data.len(),
        }
    }

    fn get(&self, pos: usize) -> Option<&Scalar> {
        if pos < self.len() {
            Some(match self {
                Row::Scalar(x) => x,
                Row::Vector(x) => &x[pos],
                Row::Tuple(x) => &x.data[pos],
            })
        } else {
            None
        }
    }

    fn iter(&self) -> Box<IterScalar<'a>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct RowPk {
    pub pk: usize,
    pub data: Vec<Scalar>,
}

impl RowPk {
    pub fn new(pk: usize, data: Vec<Scalar>) -> Self {
        RowPk { pk, data }
    }

    pub fn pk(&self) -> &Scalar {
        if self.data.is_empty() {
            &Scalar::Unit
        } else {
            &self.data[self.pk]
        }
    }
}

impl PartialEq for RowPk {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for RowPk {}

impl Hash for RowPk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl PartialOrd for RowPk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data.cmp(&other.data))
    }
}

impl Ord for RowPk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl fmt::Display for RowPk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_row(self.data.iter(), f)
    }
}

impl fmt::Display for Row<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_row(self.iter(), f)
    }
}
