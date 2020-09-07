use crate::scalar::Scalar;
use std::fmt;

pub enum Variant {
    Tag(String, i64),
    Value(String, Vec<Scalar>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SumType {
    pub tag: String,
    pub value: Box<Scalar>,
}

impl SumType {
    pub fn new(tag: &str, value: Scalar) -> Self {
        SumType {
            tag: tag.into(),
            value: Box::new(value),
        }
    }

    pub fn some(value: Scalar) -> Self {
        Self::new("Some", value)
    }

    pub fn none() -> Self {
        Self::new("None", Scalar::Unit)
    }
}

impl<T: Into<Scalar>> From<Option<T>> for SumType {
    fn from(x: Option<T>) -> Self {
        if let Some(x) = x {
            SumType::some(x.into())
        } else {
            SumType::none()
        }
    }
}

impl<T> From<SumType> for Option<T>
where
    T: From<Scalar>,
    T: From<Box<Scalar>>,
{
    fn from(x: SumType) -> Self {
        match x.tag.as_str() {
            "Some" => Some(x.value.into()),
            "None" => None,
            x => unreachable!(x),
        }
    }
}

impl fmt::Display for SumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.tag, self.value)
    }
}
