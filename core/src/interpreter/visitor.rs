use crate::interpreter::ast::*;
use crate::interpreter::Identifier;
use crate::scalar::Scalar;

pub trait Visitor<T> {
    fn visit_scalar(&mut self, of: &Scalar) -> T;
    fn visit_block(&mut self, of: &Block) -> T;
    fn visit_get_var(&mut self, of: &Identifier) -> T;
    fn visit_let(&mut self, named: &Identifier, of: &Expr) -> T;
    fn visit_var(&mut self, named: &Identifier, of: &Expr) -> T;
    fn visit_bin_op(&mut self, of: &BinaryOperation) -> T;
    fn visit_bool_op(&mut self, test: &BoolOperation) -> T;
    fn visit_if(&mut self, test: &BoolOperation, if_true: &Expr, if_false: &Expr) -> T;
    fn visit_expr(&mut self, of: &Expr) -> T;
}
