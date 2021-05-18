#![allow(clippy::unnecessary_wraps)]
use crate::prelude::*;

fn print(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        print!("{}", x);
    }
    Ok(Scalar::Unit)
}

fn print_ln(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        println!("{}", x);
    }
    Ok(Scalar::Unit)
}

macro_rules! cmp_fn {
    ($name:ident, $fun:ident) => {
        pub fn $name(row: &[Scalar]) -> Result<Scalar> {
            let lhs = &row[0];
            let rhs = &row[1];
            Ok(lhs.$fun(rhs).clone().into())
        }
    };
}

cmp_fn!(fn_min, min);
cmp_fn!(fn_max, max);

pub fn fold(init: Scalar, params: &[Scalar], f: RelFun) -> Result<Scalar> {
    let rel = &params[0];
    let schema = rel.schema();
    let mut acc = init.clone();
    for x in rel.rows_iter() {
        acc = f(&[acc, Vector::new_table(x, schema.clone()).into()])?;
    }
    Ok(acc.to_scalar().unwrap_or(init))
}

pub fn min(params: &[Scalar]) -> Result<Scalar> {
    let value = &params[0];
    if value.rows().unwrap_or(0) != 0 {
        fold(Scalar::Top, params, fn_min)
    } else {
        Ok(Scalar::Unit)
    }
}

pub fn max(params: &[Scalar]) -> Result<Scalar> {
    let value = &params[0];
    if value.rows().unwrap_or(0) != 0 {
        fold(Scalar::Unit, params, fn_max)
    } else {
        Ok(Scalar::Unit)
    }
}

fn basic_fn_variadic(name: &str, kind: DataType, f: RelFun) -> Function {
    Function::new_single(
        name,
        Param::kind(DataType::Variadic(Box::new(kind))),
        DataType::Any,
        Box::new(f),
    )
}

fn cmp_values(name: &str, param: DataType, f: RelFun) -> Function {
    Function::new_single(name, Param::kind(param.clone()), param, Box::new(f))
}

pub fn functions() -> Vec<Function> {
    vec![
        basic_fn_variadic("print", DataType::Any, print),
        basic_fn_variadic("println", DataType::Any, print_ln),
        cmp_values("min", DataType::Any, min),
        cmp_values("max", DataType::Any, max),
    ]
}
