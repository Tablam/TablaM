pub mod dsl;
pub mod refcount;
pub mod scalar;
pub mod schema;
pub mod sum_type;
pub mod types;
pub mod vector;

pub extern crate bit_vec;
pub extern crate chrono;
pub extern crate decorum;
pub extern crate rust_decimal;

mod for_impl {
    pub use std::any::Any;
    pub use std::cmp::Ordering;
    pub use std::fmt;
    pub use std::hash::Hash;
    pub use std::hash::Hasher;

    pub use crate::types::{cmp, cmp_eq};
}

pub mod prelude {
    pub use decorum::R64;
    pub use rust_decimal::Decimal;

    pub use crate::dsl::*;
    pub use crate::scalar::Scalar;
    pub use crate::schema::*;
    pub use crate::sum_type::Case;
    pub use crate::types::{Column, ColumnAlias, DataType, NativeKind, Rel, RelShape};
    pub use crate::vector::Vector;
}
