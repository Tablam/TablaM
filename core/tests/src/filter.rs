use tablam::prelude::*;

#[test]
fn test_display() {
    let rel = array(&[1, 2, 3]);
    let q = rel.query();
    assert_eq!(&format!("{}", q), "");
    let q = rel.query().eq(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 = 1");
    let q = rel.query().not_eq(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 <> 1");
    let q = rel.query().greater(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 > 1");
    let q = rel.query().greater_eq(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 >= 1");
    let q = rel.query().less(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 < 1");
    let q = rel.query().less_eq(qcol(0), qscalar(1));
    assert_eq!(&format!("{}", q), "?where #0 <= 1");
}

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);
    let q = rel.query().not_eq(qcol(0), qscalar(1));

    let (schema, xs) = q.compile(rel.rows_iter());
    let rel = Vector::from_iter(schema, xs);
    assert_eq!(&format!("{}", rel), "Vec[I64; 2; 3]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);

    let q = rel.query().not_eq(qcol(0), qscalar(1));

    let (schema, xs) = q.compile(rel.rows_iter());
    let rel = Tree::from_iter(schema, xs);
    assert_eq!(
        &format!("{}", rel),
        "Tree[pk key:I64, value:I64; 3, 4; 5, 6]"
    );
}
