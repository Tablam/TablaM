use std::ops::*;

use tablam::for_impl::*;

use tablam::cmd_impl;
use tablam::interpreter::prelude::*;
use tablam::prelude::*;

macro_rules! math_op {
    ($name:ident, $op:path) => {
        pub fn $name(params: &[Scalar]) -> ResultT<Scalar> {
            let x = &params[0];
            let y = &params[1];

            match (x, y) {
                (Scalar::I64(a), Scalar::I64(b)) => Ok(bin_op::<i64, _>($op, *a, *b)),
                (Scalar::F64(a), Scalar::F64(b)) => Ok(bin_op::<R64, _>($op, *a, *b)),
                (Scalar::Decimal(a), Scalar::Decimal(b)) => Ok(bin_op::<Decimal, _>($op, *a, *b)),
                (Scalar::Vector(data), y) => {
                    let data = data.fold_fn(y, $name)?;
                    Ok(Scalar::Vector(Rc::new(data)))
                }
                //(a, b) => fold_fn2(a, b, $name),
                _ => todo!(),
            }
        }
    };
}

math_op!(math_add, Add::add);
math_op!(math_minus, Sub::sub);
math_op!(math_mul, Mul::mul);
math_op!(math_div, Div::div);

#[derive(Clone)]
struct AddT;

impl Cmd for AddT {
    cmd_impl!("std.math", "add", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_add(params)
    }
}

#[derive(Clone)]
struct MinusT;

impl Cmd for MinusT {
    cmd_impl!("std.math", "minus", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_minus(params)
    }
}

#[derive(Clone)]
struct MulT;

impl Cmd for MulT {
    cmd_impl!("std.math", "mul", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_mul(params)
    }
}

#[derive(Clone)]
struct DivT;

impl Cmd for DivT {
    cmd_impl!("std.math", "div", BIN_MATH);
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar> {
        math_div(params)
    }
}
// pub fn fold(init: Scalar, params: &[Scalar], f: RelFun) -> ResultT<Scalar> {
//     let rel = &params[0];
//     let schema = rel.schema();
//     let mut acc = init.clone();
//     for x in rel.rows_iter() {
//         acc = f(&[acc, Vector::new_table(x, schema.clone()).into()])?;
//     }
//     Ok(acc.to_scalar().unwrap_or(init))
// }
//
// pub fn math_fn(name: &str, kind: DataType, f: RelFun) -> Function {
//     Function::new_bin_op(name, "left", "right", kind, Box::new(f))
// }
//
// pub fn sum(params: &[Scalar]) -> ResultT<Scalar> {
//     let init = params[0].kind().default_value();
//     fold(init, params, math_add)
// }
//
// pub fn avg(params: &[Scalar]) -> ResultT<Scalar> {
//     let init = params[0].kind().default_value();
//     let total = params[0].rows().unwrap_or(0);
//     let total = if total == 0 { 1 } else { total };
//
//     math_div(&[fold(init, params, math_add)?, Scalar::I64(total as i64)])
// }
//
// fn math_fold(name: &str, param: DataType, ret: DataType, f: RelFun) -> Function {
//     Function::new_single(name, Param::kind(param), ret, Box::new(f))
// }

pub fn functions() -> Mod {
    Mod::new(
        "math",
        &[
            Box::new(AddT),
            Box::new(MinusT),
            Box::new(MulT),
            Box::new(DivT),
        ],
    )
}
