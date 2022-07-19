/// Utilities for directly build TablaM types as a domain-specific-language (DSL)
use crate::prelude::*;

pub fn scalar<T: Into<Scalar>>(x: T) -> Scalar {
    x.into()
}

pub fn array<T: Into<Array>>(x: T) -> Array {
    x.into()
}

pub fn vector<T: NativeKind + Into<Array>>(x: T) -> Vector {
    let rows = T::num_rows();
    Vector::new(Schema::new_scalar(T::kind()), rows, x.into())
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
