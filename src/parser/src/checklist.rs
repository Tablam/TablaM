use crate::token::{BinaryOp, CmpOp, SepOp, Syntax, UnaryOp};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Bool,
    I64,
    Expr,
    BinOP(BinaryOp),
    Unexpected(Syntax),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Task {
    Start,
    Expr,
    BinOp(BinaryOp),
}
