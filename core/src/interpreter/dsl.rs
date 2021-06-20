use crate::interpreter::ast::{BinaryOperation, Block};
use crate::interpreter::prelude::{BoolOperation, Expr};
use crate::prelude::BinOp;
use crate::scalar::Scalar;

pub fn value<T: Into<Scalar>>(x: T) -> Expr {
    Expr::Value(x.into())
}

pub fn op_bool(x: bool) -> BoolOperation {
    BoolOperation::Bool(x)
}

pub fn if_(test: BoolOperation, if_true: Expr, if_false: Expr) -> Expr {
    Expr::If(Box::new(test), Box::new(if_true), Box::new(if_false))
}

pub fn block(lines: Vec<Expr>) -> Expr {
    Expr::Block(Block(lines))
}

pub fn set_i(name: &str, var: Expr) -> Expr {
    Expr::Immutable(name.into(), Box::new(var))
}

pub fn get(name: &str) -> Expr {
    Expr::Variable(name.into())
}

pub fn bin_op(op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
    let op = BinaryOperation::new(op, Box::new(lhs), Box::new(rhs));

    Expr::BinaryOp(op)
}

pub fn plus(lhs: Expr, rhs: Expr) -> Expr {
    bin_op(BinOp::Add, lhs, rhs)
}
