use tablam::prelude::*;

pub static PRODUCTS_CSV: &str = include_str!("../../../data/products.csv");

#[test]
fn test_mem_size() {
    //Just a sanity check to not blow up the memory!
    assert_eq!(std::mem::size_of::<Scalar>(), 24);
}

#[test]
fn cmp() {
    let a: Scalar = array(&[1, 2, 3]).into();
    assert_eq!(int(3), basic::max(&[a.clone()]).unwrap());
    assert_eq!(int(1), basic::min(&[a]).unwrap());
}
