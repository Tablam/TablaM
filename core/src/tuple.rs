use std::collections::BTreeMap;

use crate::for_impl::*;
use crate::prelude::*;
use crate::types::format_list;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelTuple {
    data: BTreeMap<String, Scalar>,
}

impl RelTuple {
    pub fn new(data: BTreeMap<String, Scalar>) -> Self {
        RelTuple { data }
    }
}

impl Rel for RelTuple {
    fn type_name(&self) -> &str {
        "Tuple"
    }

    fn kind(&self) -> DataType {
        let kind: Vec<_> = self.data.values().map(|x| x.kind()).collect();
        DataType::Tuple(kind.into())
    }

    fn schema(&self) -> Schema {
        let fields = self
            .data
            .iter()
            .map(|(k, v)| Field::new(k, v.kind()))
            .collect();

        Schema::new(fields, None)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data.len()
    }

    fn rows(&self) -> Option<usize> {
        Some(1)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        if self.len() == 1 {
            return RelShape::Scalar;
        }
        RelShape::Vec
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

impl fmt::Display for RelTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(
            self.data.iter().map(|(k, v)| format!("{}:{}", k, v)),
            self.len(),
            "|",
            "|",
            f,
        )
    }
}
