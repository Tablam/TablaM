use crate::prelude::*;
use crate::tree::Tree;

pub fn field(name: &str, kind: DataType) -> Field {
    Field::new(name, kind)
}

pub fn schema(names: &[(&str, DataType)], pk: Option<usize>) -> Schema {
    let fields = names
        .iter()
        .map(|(name, kind)| Field::new(name, kind.clone()))
        .collect();

    Schema::new(fields, pk)
}

pub fn schema_single(name: &str, kind: DataType) -> Schema {
    Schema::new_single(name, kind)
}

pub fn schema_it(kind: DataType) -> Schema {
    schema_single("it", kind)
}

pub fn schema_kv(key: DataType, value: DataType) -> Schema {
    schema(&[("key", key), ("value", value)], Some(0))
}

pub fn colp(pos: usize) -> Column {
    Column::Pos(pos)
}

pub fn coln(name: &str) -> Column {
    Column::Name(name.to_string())
}

pub fn qcol(pos: usize) -> Comparable {
    Comparable::Column(pos)
}
pub fn qscalar<T: Into<Scalar>>(x: T) -> Comparable {
    Comparable::Scalar(x.into())
}

pub fn to_vec<T>(x: &[T]) -> Vec<Scalar>
where
    T: Into<Scalar> + Clone,
{
    x.iter().cloned().map(Into::into).collect()
}

pub fn array<T>(x: &[T]) -> Vector
where
    T: Into<Scalar> + Clone + NativeKind,
{
    Vector::from_slice(x, schema_it(T::kind()))
}

pub fn narray<'a, T: 'a>(xs: impl Iterator<Item = &'a [T]>) -> Vector
where
    T: Into<Scalar> + Clone + NativeKind,
{
    Vector::from_iter(schema_it(T::kind()), xs.map(to_vec))
}

pub fn tree<'a, T: 'a>(schema: Schema, xs: impl Iterator<Item = &'a [T]>) -> Tree
where
    T: Into<Scalar> + Clone + NativeKind,
{
    Tree::from_iter(schema, xs.map(|x| to_vec(x)))
}

pub fn tree_kv<T>(data: &[T]) -> Tree
where
    T: Into<Scalar> + Clone + NativeKind,
{
    let schema = schema_kv(T::kind(), T::kind());
    let xs = data.chunks(2).map(|x| to_vec(x));
    Tree::from_iter(schema, xs)
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

pub fn scalar<T: Into<Scalar>>(x: T) -> Vector {
    Vector::new_scalar(x.into())
}

pub fn some<T: Into<Scalar>>(x: T) -> Scalar {
    Case::some(x.into()).into()
}
