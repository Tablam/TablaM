use tablam::prelude::*;

use crate::utils::*;

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

    check_query_vec(&rel, q, "Vec[it:Int; 2; 3]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);
    let q = rel.query().not_eq(qcol(0), qscalar(1));

    check_query_tree(&rel, q, "Tree[pk key:Int, value:Int; 3, 4; 5, 6]");
}

#[test]
fn test_names() {
    let rel = array(&[1, 2, 3]);
    let q = rel
        .query()
        .select(&[coln("it")])
        .eq(qcol_name("it"), qscalar(1));
    check_query_vec(&rel, q, "Vec[it:Int; 1]");
}
