use std::ops::*;

use crate::prelude::*;

macro_rules! math_op {
    ($name:ident, $op:path) => {
        pub fn $name(params: &[Scalar]) -> Result<Scalar> {
            let x = &params[0];
            let y = &params[1];

            if x.kind().kind_group() != KindGroup::Numbers
                || y.kind().kind_group() != KindGroup::Numbers
            {
                return Err(Error::TypeMismatchBinOp(x.kind(), y.kind()));
            }

            match (x, y) {
                (Scalar::I64(a), Scalar::I64(b)) => Ok(bin_op::<i64, _>($op, *a, *b)),
                (Scalar::F64(a), Scalar::F64(b)) => Ok(bin_op::<R64, _>($op, *a, *b)),
                (Scalar::Decimal(a), Scalar::Decimal(b)) => Ok(bin_op::<Decimal, _>($op, *a, *b)),
                (Scalar::Vector(data), Scalar::Decimal(_)) => {
                    let data = data.fold_fn(y, $name)?;
                    Ok(data.into())
                }
                (a, b) => fold_fn2(a, b, $name),
            }
        }
    };
}

math_op!(math_add, Add::add);
math_op!(math_minus, Sub::sub);
math_op!(math_mul, Mul::mul);
math_op!(math_div, Div::div);

fn math_fn(name: &str, kind: DataType, f: RelFun) -> Function {
    Function::new_bin_op(name, "left", "right", kind, Box::new(f))
}

pub fn math_functions() -> Vec<Function> {
    let mut fun = Vec::with_capacity(4 * 3);

    for kind in &[DataType::I64, DataType::F64, DataType::Decimal] {
        fun.push(math_fn("add", kind.clone(), math_add));
        fun.push(math_fn("minus", kind.clone(), math_minus));
        fun.push(math_fn("mul", kind.clone(), math_mul));
        fun.push(math_fn("div", kind.clone(), math_div));
    }
    fun
}
