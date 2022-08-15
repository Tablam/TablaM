use parser::ast::Ast;
use std::collections::HashMap;

pub struct Env {
    vars: HashMap<String, Ast>,
    parent: Vec<Env>,
}
