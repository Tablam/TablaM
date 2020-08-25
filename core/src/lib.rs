pub mod dsl;
pub mod errors;
pub mod function;
pub mod joins;
pub mod query;
pub mod refcount;
pub mod row;
pub mod scalar;
pub mod schema;
pub mod stdlib;
pub mod sum_type;
pub mod tree;
pub mod types;
pub mod vector;

pub extern crate bit_vec;
pub extern crate chrono;
pub extern crate decorum;
pub extern crate rust_decimal;
#[macro_use]
pub extern crate derivative;
pub extern crate derive_more;

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
    pub use itertools;
    pub use rust_decimal::Decimal;

    pub use crate::dsl::*;
    pub use crate::errors::{Error, Result};
    pub use crate::function::*;
    pub use crate::query::{Comparable, JoinOp, QueryOp};
    pub use crate::row::RowPk;
    pub use crate::scalar::{fold_fn2, Scalar};
    pub use crate::schema::*;
    pub use crate::stdlib::*;
    pub use crate::sum_type::Case;
    pub use crate::tree::Tree;
    pub use crate::types::{
        Column, ColumnAlias, DataType, KindGroup, NativeKind, Rel, RelShape, Tuple,
    };
    pub use crate::vector::Vector;
}
