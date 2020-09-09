use tablam::prelude::*;

#[test]
fn sum_type() {
    let s = SumVariant::some(1i64.into());
    assert_eq!(Some(Scalar::I64(1)), Option::from(s));

    let s = SumVariant::none();
    let x: Option<Scalar> = None;
    assert_eq!(x, Option::from(s));
}

#[test]
fn types() {
    let one = int(1);
    let s: Scalar = some(1i64);

    assert_eq!(one.kind(), DataType::I64);
    assert_eq!(s.kind(), DataType::Sum(vec![DataType::I64].into()));
}
