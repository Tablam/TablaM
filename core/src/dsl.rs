use crate::scalar::{Col, Scalar};
use crate::schema::*;
use crate::types::*;
use decorum::R64;
use rust_decimal::Decimal;

pub fn field(name: &str, kind: DataType) -> Field {
    Field::new(name, kind)
}

pub fn schema(names: &[(&str, DataType)]) -> Schema {
    let fields = names
        .iter()
        .map(|(name, kind)| Field::new(name, kind.clone()))
        .collect();

    Schema::new(fields)
}

pub fn schema_single(name: &str, kind: DataType) -> Schema {
    Schema::new_single(name, kind)
}

pub fn schema_it(kind: DataType) -> Schema {
    schema_single("it", kind)
}

pub fn colp(pos: usize) -> Column {
    Column::Pos(pos)
}

pub fn coln(name: &str) -> Column {
    Column::Name(name.to_string())
}

pub fn row<'a, T>(x: &'a [T]) -> Col
where
    &'a T: Into<Scalar> + Clone,
{
    x.iter().map(|x| x.into()).collect()
}

pub fn value<'a, T>(x: &'a [T]) -> Scalar
where
    &'a [T]: Into<Scalar>,
{
    x.into()
}

pub fn int(x: i64) -> Scalar {
    x.into()
}
pub fn str(x: &str) -> Scalar {
    x.into()
}
pub fn float(x: R64) -> Scalar {
    x.into()
}
pub fn dec(x: Decimal) -> Scalar {
    x.into()
}
