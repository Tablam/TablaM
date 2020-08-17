use tablam::prelude::*;

use crate::utils::*;

#[test]
fn test_vec() {
    let rel = array(&[2, 2, 3]);
    let q = rel.query().distinct();

    check_query_vec(&rel, q, "Vec[Int; 2; 3]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[2, 2, 2, 3, 1, 4]);
    let q = rel.query().distinct();

    check_query_tree(&rel, q, "Tree[pk key:Int, value:Int; 1, 4; 2, 2; 2, 3]");
}
