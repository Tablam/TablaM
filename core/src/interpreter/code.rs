use crate::for_impl::*;
use crate::prelude::*;

use crate::interpreter::modules::CmdBox;
use crate::interpreter::program::Env;
use derive_more::From;

pub struct CompiledExpr<'a>(Box<dyn 'a + Fn(&Env) -> ResultT<Scalar>>);

impl<'a> CompiledExpr<'a> {
    /// Creates a compiled expression IR from a generic closure.
    pub(crate) fn new(closure: impl 'a + Fn(&Env) -> ResultT<Scalar>) -> Self {
        CompiledExpr(Box::new(closure))
    }

    /// Executes a filter against a provided context with values.
    pub fn execute<'e: 'a>(&self, ctx: &'e Env) -> ResultT<Scalar> {
        self.0(ctx)
    }
}

impl fmt::Debug for CompiledExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Code()")
    }
}

#[derive(Debug, Clone, From)]
pub struct Lines(pub Vec<Code>);

#[derive(Debug, Clone)]
pub enum Code {
    Pass,
    Value(Scalar),
    Bool(BoolOp),
    If(BoolOp, Box<Code>, Box<Code>),
    BinOp(CmdBox, Vec<Code>, Vec<Code>),
    Block(Lines),
    Code(slotmap::DefaultKey),
}

impl Code {
    pub fn as_scalar(&self) -> Option<Scalar> {
        match self {
            Code::Value(x) => Some(x.clone()),
            Code::Bool(x) => x.as_bool().map(Scalar::Bool),
            Code::Block(lines) => {
                if lines.0.len() == 0 {
                    lines.0[0].as_scalar()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Some(Scalar::Bool(x)) = self.as_scalar() {
            Some(x)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoolOp {
    Bool(bool),
    Cmp(slotmap::DefaultKey),
}

impl BoolOp {
    fn as_bool(&self) -> Option<bool> {
        if let BoolOp::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }
}
