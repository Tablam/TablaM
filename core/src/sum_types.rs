use crate::for_impl::*;
use crate::prelude::*;

pub trait Variant {
    fn named(&self) -> &str;
    fn tags(&self) -> Vec<VariantTag<'_>>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantTag<'a> {
    tag: &'a str,
    values: &'a [DataType],
}

impl<'a> VariantTag<'a> {
    pub fn new(tag: &'a str, values: &'a [DataType]) -> Self {
        VariantTag { tag, values }
    }

    pub fn new_simple(tag: &'a str) -> Self {
        VariantTag { tag, values: &[] }
    }
}

impl<T: Into<Scalar>> Variant for Option<T> {
    fn named(&self) -> &str {
        "Option"
    }

    fn tags(&self) -> Vec<VariantTag<'_>> {
        vec![
            VariantTag::new_simple("None"),
            VariantTag::new_simple("Some"),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SumConstructor {
    tag: String,
    params: Vec<DataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SumVariant {
    tag: String,
    values: Vec<Scalar>,
}

impl SumVariant {
    pub fn new(tag: &str, values: &[Scalar]) -> Self {
        SumVariant {
            tag: tag.into(),
            values: values.into(),
        }
    }

    pub fn some(value: Scalar) -> Self {
        Self::new("Some", &[value])
    }

    pub fn none() -> Self {
        Self::new("None", &[])
    }

    pub fn first(&self) -> &Scalar {
        &self.values[0]
    }

    pub fn kind(&self) -> DataType {
        DataType::Sum(
            self.values
                .iter()
                .map(|x| x.kind())
                .collect::<Vec<_>>()
                .into(),
        )
    }
}

impl<T: Into<Scalar>> From<Option<T>> for SumVariant {
    fn from(x: Option<T>) -> Self {
        if let Some(x) = x {
            SumVariant::some(x.into())
        } else {
            SumVariant::none()
        }
    }
}

impl<T> From<SumVariant> for Option<T>
where
    T: From<Scalar>,
    T: From<Box<Scalar>>,
{
    fn from(x: SumVariant) -> Self {
        match x.tag.as_str() {
            "Some" => Some(x.first().clone().into()),
            "None" => None,
            x => unreachable!(x),
        }
    }
}

impl fmt::Display for SumVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.tag)?;

        match self.values.len() {
            0 => Ok(()),
            1 => write!(f, "({})", self.first()),
            _ => {
                write!(f, "(")?;
                fmt_row(self.values.iter(), f)?;
                write!(f, ")")
            }
        }
    }
}
