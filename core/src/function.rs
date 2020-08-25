use core::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use derive_more::{Display, From};

use crate::prelude::*;
use crate::types::format_list;

//pub type RelFun = fn(&[Scalar]) -> Result<Scalar>;
pub type RelFun = for<'a> fn(&'a [&'a Scalar]) -> Result<Scalar>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "{} :{}", name, kind)]
pub struct Param {
    name: String,
    kind: DataType,
}

impl Param {
    pub fn new(name: &str, kind: DataType) -> Self {
        Param {
            name: name.to_string(),
            kind,
        }
    }

    pub fn kind(kind: DataType) -> Self {
        Param {
            name: "".to_string(),
            kind,
        }
    }
}

#[derive(Clone, From)]
pub struct Function {
    name: String,
    params: Vec<Param>,
    result: Vec<Param>,
    f: Box<RelFun>,
}

impl Function {
    pub fn new(name: &str, params: &[Param], result: &[Param], f: Box<RelFun>) -> Self {
        Function {
            name: name.to_string(),
            params: params.to_vec(),
            result: result.to_vec(),
            f,
        }
    }

    pub fn new_bin_op(name: &str, left: &str, right: &str, kind: DataType, f: Box<RelFun>) -> Self {
        let lhs = Param::new(left, kind.clone());
        let rhs = Param::new(right, kind.clone());
        let ret = Param::new("", kind);

        Self::new(name, &[lhs, rhs], &[ret], f)
    }

    pub fn new_single(name: &str, param: Param, ret: DataType, f: Box<RelFun>) -> Self {
        let ret = Param::new("", ret);

        Self::new(name, &[param], &[ret], f)
    }

    pub fn call(&self, params: &[&Scalar]) -> Result<Scalar> {
        if params.len() != self.params.len() {
            return Err(Error::ParamCount(params.len(), self.params.len()));
        }
        (self.f)(params)
    }

    pub fn key(&self) -> String {
        let mut key = String::new();
        key += &self.name;
        for p in &self.params {
            key += &*format!("_{}", p.kind);
        }
        key
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
