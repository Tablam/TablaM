use std::mem::swap;
use tablam::prelude::*;
use tablam::stdlib::math::functions;

#[test]
fn test_scalar() {
    let a = 1.into();
    let b = 2.into();

    assert_eq!(int(3), math::math_add(&[a, b]).unwrap());
}

#[test]
fn test_scalar_vec() {
    let mut a = int(1);
    let mut b: Scalar = array(&[1, 2, 3]).into();
    let result: Scalar = array(&[1, 0, 0]).into();

    assert_eq!(result, math::math_div(&[a.clone(), b.clone()]).unwrap());
    //Check commutativity
    swap(&mut a, &mut b);
    assert_eq!(result, math::math_div(&[a, b]).unwrap());
}

#[test]
fn test_vec_vec() {
    let a = array(&[1, 2, 3]).into();
    let b = array(&[1, 2, 3]).into();

    let result: Scalar = array(&[1, 4, 9]).into();

    assert_eq!(result, math::math_mul(&[a, b]).unwrap());
}

#[test]
fn test_vec_vec_invalid() {
    let a = array(&[1, 2, 3]).into();
    let b = array(&[1, 2]).into();

    match math::math_add(&[a, b]) {
        Err(Error::RankNotMatch) => (),
        e => assert!(false, "Fail to report err: {:?}", e),
    }
}

#[test]
fn test_invalid_op() {
    let a = 1.into();
    let b = "a".into();
    match math::math_add(&[a, b]) {
        Err(Error::TypeMismatchBinOp(_, _)) => (),
        _ => assert!(false, "Fail to report err"),
    }
}

#[test]
fn test_functions() {
    let a = 1.into();
    let b = 2.into();

    let f = functions();
    let plus = &f[0];

    dbg!(plus.key());
    let result = plus.call(&[a, b]).unwrap();
    assert_eq!(int(3), result);
}

#[test]
fn test_folds() {
    let a = 1.into();
    assert_eq!(int(1), math::sum(&[a]).unwrap());
    let a = array(&[1, 2, 3]).into();
    assert_eq!(int(6), math::sum(&[a]).unwrap());

    let a = array(&[1, 2, 3]).into();
    assert_eq!(int(2), math::avg(&[a]).unwrap());
}
