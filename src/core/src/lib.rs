pub extern crate chrono;
pub extern crate decorum;
pub extern crate derive_more;
pub extern crate rust_decimal;
pub extern crate slotmap;

pub mod algebraic;
pub mod convert;
pub mod dsl;
pub mod errors;
pub mod relation;
pub mod row;
pub mod scalar;
pub mod schema;
pub mod types;
pub mod utils;
pub mod vector;

pub mod extra_types {
    pub use bitvec::prelude as bv;
    pub use chrono;
    pub use decorum;
    pub use decorum::R64;
    pub use rust_decimal::Decimal;
}

pub mod prelude {
    pub use crate::dsl;
    pub use crate::errors::{Error, ErrorLang, ResultT};
    pub use crate::extra_types::*;
    pub use crate::relation::Rel;
    pub use crate::scalar::{DateKind, DateT, Scalar, ScalarSlice};
    pub use crate::schema::*;
    pub use crate::types::*;
    pub use crate::utils::*;
    pub use crate::vector::{Array, VecPos, Vector};
}