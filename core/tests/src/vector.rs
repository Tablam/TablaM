use tablam::prelude::*;

#[test]
fn test_scalar() {
    let rel = scalar(1);
    assert!(rel.is_scalar());
    assert_eq!(rel.len(), 1);
    assert_eq!(rel.rows(), Some(1));
    assert_eq!(rel.cols(), 1);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0), &[int(1)]);
    assert_eq!(rel.col(0), scalar(1));

    assert_eq!(&format!("{}", rel), "Vec[I64; 1]");
}

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);
    assert!(!rel.is_scalar());
    assert_eq!(rel.len(), 3);
    assert_eq!(rel.rows(), Some(3));
    assert_eq!(rel.cols(), 1);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0), &[int(1)]);
    assert_eq!(rel.col(0), rel.clone());

    assert_eq!(&format!("{}", rel), "Vec[I64; 1; 2; 3]");
}

#[test]
fn test_table() {
    let rel = narray([1, 2, 3, 4, 5, 6].chunks(3));
    assert!(!rel.is_scalar());
    assert_eq!(rel.len(), 6);
    assert_eq!(rel.rows(), Some(2));
    assert_eq!(rel.cols(), 3);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0), &[int(1), int(2), int(3)]);
    assert_eq!(rel.row(1), &[int(4), int(5), int(6)]);
    //assert_eq!(rel.col_iter(0).collect(), rel.clone());

    assert_eq!(&format!("{}", rel), "Vec[I64; 1, 2, 3; 4, 5, 6]");
}
