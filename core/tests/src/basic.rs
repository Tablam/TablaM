use tablam::prelude::*;

pub static PRODUCTS_CSV: &str = include_str!("../../../data/products.csv");

#[test]
fn test_mem_size() {
    //Just a sanity check to not blow up the memory!
    assert_eq!(std::mem::size_of::<Scalar>(), 24);
}

#[test]
fn sum_type() {
    let s = SumType::some(1i64.into());
    assert_eq!(Some(Scalar::I64(1)), Option::from(s));
}

#[test]
fn types() {
    let one = int(1);
    let s: Scalar = some(1i64);

    assert_eq!(one.kind(), DataType::I64);
    assert_eq!(s.kind(), DataType::Sum(Box::new(DataType::I64)));
}

#[test]
fn cmp() {
    let a: Scalar = array(&[1, 2, 3]).into();
    assert_eq!(int(3), basic::max(&[a.clone()]).unwrap());
    assert_eq!(int(1), basic::min(&[a]).unwrap());
}
