use tablam::prelude::*;

#[test]
fn test_empty() {
    let rel = Tree::empty(schema_kv(DataType::I64, DataType::I64));
    assert_eq!(rel.rel_shape(), RelShape::Table);
    assert_eq!(rel.len(), 0);
    assert_eq!(rel.rows(), Some(0));
    assert_eq!(rel.cols(), 2);
    assert_eq!(
        rel.kind(),
        DataType::Tree(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(&format!("{}", rel), "Tree[pk key:I64, value:I64;]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(rel.rel_shape(), RelShape::Table);
    assert_eq!(rel.len(), 6);
    assert_eq!(rel.rows(), Some(3));
    assert_eq!(rel.cols(), 2);
    assert_eq!(
        rel.kind(),
        DataType::Tree(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(
        &format!("{}", rel),
        "Tree[pk key:I64, value:I64; 1, 2; 3, 4; 5, 6]"
    );
}

#[test]
fn test_iter() {
    let rel = tree_kv(&[1, 2]);

    let first = rel.rows_iter().next();
    assert_eq!(Some([int(1), int(2)].as_ref()), first);

    let first = rel.col_iter(1).next();
    assert_eq!(Some([int(2)].as_ref()), first);
}
