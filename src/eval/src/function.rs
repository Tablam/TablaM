use std::fmt;

use crate::code::Code;
use crate::env::Env;

/// A `index` into the list of [Fun]
pub type FunctionId = u32;

pub trait FunVM: for<'a> Fn(&'a Env) -> Code {
    fn clone_object(&self) -> Box<dyn FunVM>;
}

impl<F> FunVM for F
where
    F: 'static + Clone + for<'a> Fn(&'a Env) -> Code,
{
    fn clone_object(&self) -> Box<dyn FunVM> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FunVM> {
    fn clone(&self) -> Self {
        self.clone_object()
    }
}

pub struct FunVm {
    pub(crate) idx: FunctionId,
    pub(crate) fun: Box<dyn FunVM>,
}

impl FunVm {
    pub fn new(idx: FunctionId, fun: Box<dyn FunVM>) -> Self {
        Self { idx, fun }
    }
    pub fn call(&self, env: &Env) -> Code {
        (self.fun)(env)
    }
}

impl fmt::Debug for FunVm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fun({})", self.idx)
    }
}
