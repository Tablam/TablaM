use std::ops::*;

use crate::cmd_impl;
use crate::for_impl::*;
use crate::interpreter::prelude::*;
use crate::prelude::*;

macro_rules! math_op {
    ($name:ident, $op:path) => {
        pub fn $name(params: &[Scalar]) -> ResultT<Scalar> {
            let x = &params[0];
            let y = &params[1];

            match (x, y) {
                (Scalar::I64(a), Scalar::I64(b)) => Ok(bin_op::<i64, _>($op, *a, *b)),
                (Scalar::F64(a), Scalar::F64(b)) => Ok(bin_op::<R64, _>($op, *a, *b)),
                (Scalar::Decimal(a), Scalar::Decimal(b)) => Ok(bin_op::<Decimal, _>($op, *a, *b)),
                // (Scalar::Vector(data), Scalar::Decimal(_)) => {
                //     let data = data.fold_fn(y, $name)?;
                //     Ok(data.into())
                // }
                //(a, b) => fold_fn2(a, b, $name),
                _ => todo!(),
            }
        }
    };
}

math_op!(math_add, Add::add);
math_op!(math_minus, Sub::sub);
// math_op!(math_mul, Mul::mul);
// math_op!(math_div, Div::div);

#[derive(Clone)]
struct AddT;

impl Cmd for AddT {
    cmd_impl!("std.ops", "add", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_add(params)
    }
}

#[derive(Clone)]
struct MinusT;

impl Cmd for MinusT {
    cmd_impl!("std.ops", "minus", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_minus(params)
    }
}

pub fn mod_ops() -> Mod {
    Mod::new("std.ops", &[Box::new(AddT), Box::new(MinusT)])
}
