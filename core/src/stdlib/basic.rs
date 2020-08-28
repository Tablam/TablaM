use crate::prelude::*;

fn print(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        print!("{}", x);
    }
    Ok(Scalar::None)
}

fn print_ln(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        println!("{}", x);
    }
    Ok(Scalar::None)
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
        Ok(Scalar::None)
    }
}

pub fn max(params: &[Scalar]) -> Result<Scalar> {
    let value = &params[0];
    if value.rows().unwrap_or(0) != 0 {
        fold(Scalar::None, params, fn_max)
    } else {
        Ok(Scalar::None)
    }
}

fn basic_fn_variadic(name: &str, kind: DataType, f: RelFun) -> Function {
    Function::new_single(
        name,
        Param::kind(DataType::Variadic(Box::new(kind))),
        DataType::ANY,
        Box::new(f),
    )
}

fn cmp_values(name: &str, param: DataType, f: RelFun) -> Function {
    Function::new_single(name, Param::kind(param.clone()), param, Box::new(f))
}

pub fn functions() -> Vec<Function> {
    vec![
        basic_fn_variadic("print", DataType::ANY, print),
        basic_fn_variadic("println", DataType::ANY, print_ln),
        cmp_values("min", DataType::ANY, min),
        cmp_values("max", DataType::ANY, max),
    ]
}
