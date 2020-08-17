use std::collections::HashMap;
use tablam::prelude::*;

pub enum Expression {
    Value(Scalar),
}

pub struct Environment {
    pub vars: HashMap<String, Expression>,
    pub functions: HashMap<String, Expression>,
    pub parent: Option<Box<Environment>>,
}
