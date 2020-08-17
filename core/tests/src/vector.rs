use tablam::prelude::*;

#[test]
fn test_scalar() {
    let rel = scalar(1);
    assert_eq!(rel.rel_shape(), RelShape::Scalar);
    assert_eq!(rel.len(), 1);
    assert_eq!(rel.rows(), Some(1));
    assert_eq!(rel.cols(), 1);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0), &[int(1)]);
    assert_eq!(rel.col(0), scalar(1));

    assert_eq!(&format!("{}", rel), "Vec[Int; 1]");
}

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);
    assert_eq!(rel.rel_shape(), RelShape::Vec);
    assert_eq!(rel.len(), 3);
    assert_eq!(rel.rows(), Some(3));
    assert_eq!(rel.cols(), 1);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0), &[int(1)]);
    assert_eq!(rel.col(0), rel);

    assert_eq!(&format!("{}", rel), "Vec[Int; 1; 2; 3]");
}

#[test]
fn test_table() {
    let rel = narray(3, [1, 2, 3, 4, 5, 6].chunks(3));
    assert_eq!(rel.rel_shape(), RelShape::Table);
    assert_eq!(rel.len(), 6);
    assert_eq!(rel.rows(), Some(2));
    assert_eq!(rel.cols(), 3);
    assert_eq!(
        rel.schema,
        schema(
            &[
                ("col_0", DataType::I64),
                ("col_1", DataType::I64),
                ("col_2", DataType::I64)
            ],
            None
        )
    );
    assert_eq!(
        rel.kind(),
        DataType::Vec(vec![DataType::I64, DataType::I64, DataType::I64].into())
    );
    assert_eq!(rel.row(0), &[int(1), int(2), int(3)]);
    assert_eq!(rel.row(1), &[int(4), int(5), int(6)]);

    assert_eq!(&format!("{}", rel), "Vec[Int, Int, Int; 1, 2, 3; 4, 5, 6]");
}

#[test]
fn test_iter() {
    let rel = scalar(1);
    let rows: Vec<_> = rel.rows_iter().collect();
    assert_eq!(rows, vec![&[int(1)]]);

    let rel = array::<i64>(&[]);
    let rows: Vec<_> = rel.rows_iter().collect();
    let empty: Vec<&[Scalar]> = Vec::new();
    assert_eq!(rows, empty);

    let rel = array(&[1, 2, 3]);
    let rows: Vec<_> = rel.rows_iter().collect();
    assert_eq!(rows, vec![&[int(1)], &[int(2)], &[int(3)]]);

    let rel = narray(3, [1, 2, 3, 4, 5, 6].chunks(3));
    let rows: Vec<_> = rel.rows_iter().collect();
    assert_eq!(
        rows,
        vec![&[int(1), int(2), int(3)], &[int(4), int(5), int(6)]]
    );

    let col: Vec<_> = rel.col_iter(1).collect();
    assert_eq!(col, vec![&[int(2)], &[int(4)], &[int(6)]]);
}
