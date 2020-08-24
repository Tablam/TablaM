use std::ops::*;

use crate::prelude::*;

macro_rules! math_op {
    ($name:ident, $op:path) => {
        pub fn $name(x: &Scalar, y: &Scalar) -> Result<Scalar> {
            if x.kind().kind_group() != KindGroup::Numbers
                || y.kind().kind_group() != KindGroup::Numbers
            {
                return Err(Error::InvalidTypeMath);
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

pub fn vector_math(x: &Scalar, y: &Scalar) -> Result<Scalar> {
    match (x, y) {
        (Scalar::I64(_), Scalar::Vector(data)) => {
            let data = data.fold_fn(x, math_add)?;
            Ok(data.into())
        }

        (a, b) => panic!("Argument {:?} <> {:?}", a, b),
    }
}

math_op!(math_add, Add::add);
math_op!(math_minus, Sub::sub);
math_op!(math_mul, Mul::mul);
math_op!(math_div, Div::div);
