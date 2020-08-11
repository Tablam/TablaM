use crate::scalar::Scalar;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Case {
    pub tag: String,
    pub value: Box<Scalar>,
}

impl Case {
    pub fn new(tag: &str, value: Scalar) -> Self {
        Case {
            tag: tag.into(),
            value: Box::new(value),
        }
    }

    pub fn some(value: Scalar) -> Self {
        Self::new("Some", value)
    }

    pub fn none() -> Self {
        Self::new("None", Scalar::None)
    }
}

impl<T: Into<Scalar>> From<Option<T>> for Case {
    fn from(x: Option<T>) -> Self {
        if let Some(x) = x {
            Case::some(x.into())
        } else {
            Case::none()
        }
    }
}

impl<T> From<Case> for Option<T>
where
    T: From<Scalar>,
    T: From<Box<Scalar>>,
{
    fn from(x: Case) -> Self {
        match x.tag.as_str() {
            "Some" => Some(x.value.into()),
            "None" => None,
            x => unreachable!(x),
        }
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.tag, self.value)
    }
}
