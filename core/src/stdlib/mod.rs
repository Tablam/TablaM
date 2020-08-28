use crate::function::Function;
use crate::scalar::Scalar;

pub mod basic;
pub mod io;
pub mod math;

pub fn bin_op<T, Op>(op: Op, x: T, y: T) -> Scalar
where
    Op: Fn(T, T) -> T,
    T: From<Scalar>,
    Scalar: From<T>,
{
    op(x, y).into()
}

pub fn std_functions() -> Vec<Function> {
    let mut funs = Vec::new();

    funs.extend_from_slice(&basic::functions());
    funs.extend_from_slice(&math::functions());
    funs.extend_from_slice(&io::functions());

    funs
}
