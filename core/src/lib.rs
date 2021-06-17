pub extern crate bit_vec;
pub extern crate chrono;
pub extern crate decorum;
pub extern crate derivative;
pub extern crate derive_more;
pub extern crate ndarray;
pub extern crate rust_decimal;
pub extern crate slotmap;
#[macro_use]
extern crate lazy_static;

mod dsl;
pub mod errors;
mod file;
pub mod function;
pub mod interpreter;
mod iterators;
mod map;
pub mod query;
mod refcount;
mod relation;
mod row;
mod scalar;
mod schema;
mod seq;
mod sum_types;
mod tree;
mod types;
mod utils;
mod vector;

pub mod for_impl {
    pub use decorum::R64;
    pub use rust_decimal::Decimal;
    pub use std::any::Any;
    pub use std::cmp::Ordering;
    pub use std::collections::{HashMap, HashSet};
    pub use std::fmt;
    pub use std::hash::Hash;
    pub use std::hash::Hasher;
    pub use std::rc::Rc;
}

pub mod prelude {
    pub use crate::dsl::*;
    pub use crate::errors::{Error, ResultT};
    pub use crate::file::*;
    pub use crate::function::*;
    pub use crate::map::Map;
    pub use crate::query::*;
    pub use crate::relation::{Rel, RelationDyn, ToHash};
    pub use crate::row::*;
    pub use crate::scalar::{Date, DateTime, Scalar, Time};
    pub use crate::schema::{check_pk, Field, Schema};
    pub use crate::seq::Seq;
    pub use crate::sum_types::*;
    pub use crate::tree::Tree;
    pub use crate::types::*;
    pub use crate::utils::*;
    pub use crate::vector::Vector;
}
