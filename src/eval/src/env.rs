use crate::prelude::*;
use std::collections::HashMap;

pub struct Env {
    vars: HashMap<String, Expr>,
    parent: Vec<Env>,
}
