use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, Clone, Ord)]
pub struct RowPk {
    pub pk: usize,
    pub data: Vec<Scalar>,
}

impl RowPk {
    pub fn new(pk: usize, data: Vec<Scalar>) -> Self {
        RowPk { pk, data }
    }

    pub fn pk(&self) -> &Scalar {
        &self.data[self.pk]
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
        Some(self.pk().cmp(&other.pk()))
    }
}

pub fn fmt_row(row: &[Scalar], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (pos, x) in row.iter().enumerate() {
        write!(f, " {}", x)?;
        if pos < row.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

impl fmt::Display for RowPk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_row(&self.data, f)
    }
}
