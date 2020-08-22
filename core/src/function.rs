use core::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use derive_more::From;

use crate::prelude::*;
use crate::types::format_list;

pub type RelFun = fn(&[Scalar]) -> Result<Scalar>;

#[derive(Clone, From)]
pub struct Function {
    name: String,
    params: Vec<DataType>,
    result: Vec<DataType>,
    f: Box<RelFun>,
}

impl Function {
    pub fn call(&self, params: &[Scalar]) -> Result<Scalar> {
        (self.f)(params)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fun {}", self.name)?;
        format_list(&self.params, self.params.len(), "(", ")", f)?;
        format_list(&self.result, self.params.len(), " = ", "", f)?;
        Ok(())
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fun {}({:?})={:?}", self.name, self.params, self.result)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params && self.result == other.result
    }
}

impl Eq for Function {}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.name
                .cmp(&other.name)
                .then(self.params.cmp(&other.params))
                .then(self.result.cmp(&other.result)),
        )
    }
}

impl Ord for Function {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then(self.params.cmp(&other.params))
            .then(self.result.cmp(&other.result))
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.params.hash(state);
        self.result.hash(state);
    }
}
