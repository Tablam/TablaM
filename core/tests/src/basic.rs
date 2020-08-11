use tablam::prelude::*;

#[test]
fn test_mem_size() {
    //Just a sanity check to not blow up the memory!
    assert_eq!(std::mem::size_of::<Scalar>(), 24);
}

#[test]
fn sum_type() {
    let s = Case::some(1i64.into());
    dbg!(&s);
    println!("{}", &s);
    assert_eq!(Some(Scalar::I64(1)), Option::from(s));
}

#[test]
fn types() {
    let one = int(1);
    let s: Scalar = some(1i64);

    assert_eq!(one.kind(), DataType::I64);
    assert_eq!(s.kind(), DataType::Sum(Box::new(DataType::I64)));
}
