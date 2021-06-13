use tablam::prelude::*;

#[test]
fn test_scalar() {
    let rel = scalar(1);
    assert_eq!(rel.len(), 1);
    assert_eq!(rel.size(), ShapeLen::Scalar);
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::I64);
    assert_eq!(rel.row(0).as_slice().unwrap(), &[int(1)]);
    assert_eq!(rel.col(0).iter.cloned().collect::<Vec<_>>(), &[int(1)]);

    assert_eq!(&format!("{}", rel), "Vec[it:Int; 1]");
}

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);
    assert_eq!(rel.len(), 3);
    assert_eq!(rel.size(), ShapeLen::Vec(3));
    assert_eq!(rel.schema, schema_it(DataType::I64));
    assert_eq!(rel.kind(), DataType::Vec(Box::new(DataType::I64)));
    assert_eq!(rel.row(0).as_slice().unwrap(), &[int(1)]);
    assert_eq!(rel.col(0).iter.cloned().collect::<Vec<_>>(), &[int(1)]);

    assert_eq!(&format!("{}", rel), "Vec[it:Int; 1; 2; 3]");
}

#[test]
fn test_table() {
    let rel = narray(3, [1, 2, 3, 4, 5, 6].iter());
    assert_eq!(rel.size(), ShapeLen::Table(3, 2));
    assert_eq!(rel.len(), 6);
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
        DataType::Vec2d(vec![DataType::I64, DataType::I64, DataType::I64].into())
    );
    assert_eq!(rel.row(0).as_slice().unwrap(), &[int(1), int(2), int(3)]);
    assert_eq!(rel.row(1).as_slice().unwrap(), &[int(4), int(5), int(6)]);

    assert_eq!(
        &format!("{}", rel),
        "Vec[col_0:Int, col_1:Int, col_2:Int; 1, 2, 3; 4, 5, 6]"
    );
}

#[test]
fn test_iter() {
    let rel = scalar(1);
    let rows = rows_to_vec(&rel);
    assert_eq!(rows, vec![&[int(1)]]);

    let rel = array::<i64>(&[]);
    let rows = rows_to_vec(&rel);
    let empty: Vec<&[Scalar]> = Vec::new();
    assert_eq!(rows, empty);

    let rel = array(&[1, 2, 3]);
    let rows = rows_to_vec(&rel);
    assert_eq!(rows, vec![&[int(1)], &[int(2)], &[int(3)]]);

    let rel = narray(3, [1, 2, 3, 4, 5, 6].iter());
    let rows = rows_to_vec(&rel);
    assert_eq!(
        rows,
        vec![&[int(1), int(2), int(3)], &[int(4), int(5), int(6)]]
    );

    let col: Vec<_> = rel.col(1).iter.cloned().collect();
    assert_eq!(col, vec![int(2), int(5)]);
}
