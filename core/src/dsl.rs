use crate::prelude::*;

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

pub fn array<T>(x: &[T]) -> Vector
where
    T: Into<Scalar> + Clone + NativeKind,
{
    Vector::from_slice(x, schema_it(T::kind()))
}

pub fn narray<'a, T: 'a>(x: impl Iterator<Item = &'a [T]>) -> Vector
where
    T: Into<Scalar> + Clone + NativeKind,
{
    Vector::from_iter(x, schema_it(T::kind()))
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
