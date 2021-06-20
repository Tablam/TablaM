use crate::for_impl::*;
use crate::prelude::*;

use crate::interpreter::modules::CmdBox;
use crate::interpreter::program::Env;
use derive_more::From;

pub trait CompiledExpr {
    /// Clones the command in a box reference.
    fn clone_and_box(&self) -> Compiled;
    /// Execute the code with the env
    fn execute(&self, ctx: &Env) -> ResultT<Scalar>;
}

/// Defines a box reference for a compiled expression.
pub type Compiled = Box<dyn CompiledExpr>;

impl Clone for Box<dyn CompiledExpr> {
    fn clone(&self) -> Box<dyn CompiledExpr> {
        self.clone_and_box()
    }
}

impl fmt::Debug for dyn CompiledExpr {
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
    Code(Compiled),
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
    Cmp(Compiled),
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
