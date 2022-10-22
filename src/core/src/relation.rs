use downcast_rs::{impl_downcast, Downcast};
use std::fmt;

use crate::prelude::*;

pub trait Rel: Downcast + fmt::Debug {
    fn type_name(&self) -> &str;

    fn schema(&self) -> Schema;
}

impl_downcast!(Rel);
