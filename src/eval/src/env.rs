use crate::function::FunVm;

#[derive(Debug)]
pub struct Env {
    pub(crate) fun: Vec<FunVm>,
    parent: Vec<Env>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            fun: vec![],
            parent: vec![],
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
