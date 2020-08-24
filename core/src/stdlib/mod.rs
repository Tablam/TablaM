use crate::scalar::Scalar;

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
