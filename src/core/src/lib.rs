pub extern crate chrono;
pub extern crate decorum;
pub extern crate derive_more;
pub extern crate rust_decimal;
pub extern crate slotmap;
pub extern crate text_size;
pub extern crate tree_flat;

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
    pub use chrono;
    pub use decorum;
    pub use decorum::Total;
    pub use rust_decimal::Decimal;
}

pub mod prelude {
    pub use crate::dsl;
    pub use crate::errors::{ErrorCore, ErrorLang, ResultT, Span};
    pub use crate::extra_types::*;
    pub use crate::relation::Rel;
    pub use crate::scalar::{DateKind, DateT, Scalar, ScalarSlice, F64};
    pub use crate::schema::*;
    pub use crate::types::*;
    pub use crate::utils::*;
    pub use crate::vector::{Array, VecPos, Vector};
}
