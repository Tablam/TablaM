use std::mem::swap;
use tablam::prelude::*;

#[test]
fn test_scalar() {
    let a = 1.into();
    let b = 2.into();

    assert_eq!(int(3), math::math_add(&a, &b).unwrap());
}

#[test]
fn test_scalar_vec() {
    let mut a = 1.into();
    let mut b = array(&[1, 2, 3]).into();
    let result: Scalar = array(&[1, 0, 0]).into();

    assert_eq!(result, math::math_div(&a, &b).unwrap());
    //Check commutativity
    swap(&mut a, &mut b);
    assert_eq!(result, math::math_div(&a, &b).unwrap());
}

#[test]
fn test_vec_vec() {
    let a = array(&[1, 2, 3]).into();
    let b = array(&[1, 2, 3]).into();

    let result: Scalar = array(&[1, 4, 9]).into();

    assert_eq!(result, math::math_mul(&a, &b).unwrap());
}

#[test]
fn test_vec_vec_invalid() {
    let a = array(&[1, 2, 3]).into();
    let b = array(&[1, 2]).into();

    match math::math_add(&a, &b) {
        Err(Error::RankNotMatch) => (),
        e => assert!(false, "Fail to report err: {:?}", e),
    }
}

#[test]
fn test_invalid_op() {
    let a = 1.into();
    let b = "a".into();
    match math::math_add(&a, &b) {
        Err(Error::InvalidTypeMath) => (),
        _ => assert!(false, "Fail to report err"),
    }
}
