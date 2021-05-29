use core::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use derive_more::{Display, From};

use crate::prelude::*;
use crate::types::{cmp, cmp_eq};
use std::any::Any;

//pub type RelFun = fn(&[Scalar]) -> Result<Scalar>;
pub type RelFun = for<'a> fn(&'a [Scalar]) -> Result<Scalar>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Param {
    pub name: String,
    pub kind: DataType,
}

impl Param {
    pub fn new(name: &str, kind: DataType) -> Self {
        Param {
            name: name.to_string(),
            kind,
        }
    }

    pub fn from_str(name: &str, kind: &str) -> Self {
        Param {
            name: name.to_string(),
            kind: DataType::from_str(kind).expect("DataType not implemented"),
        }
    }

    pub fn kind(kind: DataType) -> Self {
        Param {
            name: "".to_string(),
            kind,
        }
    }
}

impl From<&Param> for Field {
    fn from(x: &Param) -> Self {
        Field::new(&x.name, x.kind.clone())
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "{}", &self.kind)
        } else {
            write!(f, "{}: {}", &self.name, &self.kind)
        }
    }
}

#[derive(Clone, From, Display)]
#[display(fmt = "Fun()")]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub result: Vec<Param>,
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

    pub fn call(&self, params: &[Scalar]) -> Result<Scalar> {
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

impl Rel for Function {
    fn type_name(&self) -> &str {
        "Fun"
    }

    fn kind(&self) -> DataType {
        DataType::Fun(self.into())
    }

    fn schema(&self) -> Schema {
        Schema::new(self.params.iter().map(|x| x.into()).collect(), None)
    }

    fn len(&self) -> usize {
        0
    }

    fn cols(&self) -> usize {
        self.params.len()
    }

    fn rows(&self) -> Option<usize> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        RelShape::Table
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher)
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
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
