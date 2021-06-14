use crate::for_impl::*;
use crate::ndarray::ArrayView1;
use crate::prelude::*;

pub struct Col<'a> {
    pub pos: usize,
    pub iter: Box<IterScalar<'a>>,
}

impl<'a> Col<'a> {
    pub fn new(pos: usize, iter: Box<IterScalar<'a>>) -> Self {
        Col { pos, iter }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Row<'a> {
    Scalar(&'a Scalar),
    Vector(ArrayView1<'a, Scalar>),
    Tuple(&'a RowPk),
    Joined(Vec<Scalar>),
}

impl<'a> Row<'a> {
    fn len(&self) -> usize {
        match self {
            Row::Scalar(x) => x.len(),
            Row::Vector(x) => x.len(),
            Row::Tuple(x) => x.data.len(),
            Row::Joined(x) => x.len(),
        }
    }

    pub fn get(&self, pos: usize) -> Option<&Scalar> {
        if pos < self.len() {
            match self {
                Row::Scalar(x) => Some(x),
                Row::Vector(x) => Some(&x[pos]),
                Row::Tuple(x) => Some(&x.data[pos]),
                Row::Joined(x) => x.get(pos),
            }
        } else {
            None
        }
    }

    pub fn to_vec(&self) -> Vec<Scalar> {
        match self {
            Row::Scalar(x) => vec![(**x).clone()],
            Row::Vector(x) => x.into_iter().cloned().collect(),
            Row::Tuple(x) => x.data.clone(),
            Row::Joined(x) => x.clone(),
        }
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
        match self {
            Row::Scalar(x) => write!(f, " {}", x),
            Row::Vector(x) => fmt_row(x.into_iter(), f),
            Row::Tuple(x) => fmt_row(x.data.iter(), f),
            Row::Joined(x) => fmt_row(x.iter(), f),
        }
    }
}
