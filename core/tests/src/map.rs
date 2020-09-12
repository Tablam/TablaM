use tablam::prelude::*;

#[test]
fn test_empty() {
    let rel = Map::empty(schema_kv(DataType::I64, DataType::I64));
    assert_eq!(rel.rel_shape(), RelShape::Table);
    assert_eq!(rel.len(), 0);
    assert_eq!(rel.rows(), Some(0));
    assert_eq!(rel.cols(), 2);
    assert_eq!(
        rel.kind(),
        DataType::Map(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(&format!("{}", rel), "Map[pk key:Int, value:Int;]");
}

#[test]
fn test_tree() {
    let rel = map_kv(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(rel.rel_shape(), RelShape::Table);
    assert_eq!(rel.len(), 6);
    assert_eq!(rel.rows(), Some(3));
    assert_eq!(rel.cols(), 2);
    assert_eq!(
        rel.kind(),
        DataType::Map(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(
        &format!("{}", rel),
        "Map[pk key:Int, value:Int; 1, 2; 3, 4; 5, 6]"
    );
}

#[test]
fn test_iter() {
    let rel = map_kv(&[1, 2]);

    let first = rel.rows_iter().next();
    assert_eq!(Some([int(1), int(2)].to_vec()), first);

    let first = rel.col_iter(1).next();
    assert_eq!(Some([int(2)].to_vec()), first);
}
