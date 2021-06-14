use tablam::prelude::*;

#[test]
fn test_empty() {
    let rel = Tree::empty(schema_kv(DataType::I64, DataType::I64));
    assert_eq!(rel.size(), ShapeLen::Table(2, 0));
    assert_eq!(rel.len(), 0);
    assert_eq!(
        rel.kind(),
        DataType::Tree(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(&format!("{}", rel), "Tree[pk key:Int, value:Int]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(rel.size(), ShapeLen::Table(2, 3));
    assert_eq!(rel.len(), 6);
    assert_eq!(
        rel.kind(),
        DataType::Tree(vec![DataType::I64, DataType::I64].into())
    );

    assert_eq!(
        &format!("{}", rel),
        "Tree[pk key:Int, value:Int; 1, 2; 3, 4; 5, 6]"
    );
}

#[test]
fn test_iter() {
    let rel = tree_kv(&[1, 2]);

    let first = &rows_to_vec(&rel)[0];
    assert_eq!(&[int(1), int(2)], first.as_slice());

    let first: Vec<_> = rel.col(1).iter.cloned().collect();
    assert_eq!(&[int(2)], first.as_slice());
}
