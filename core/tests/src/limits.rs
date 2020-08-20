use tablam::prelude::*;

use crate::utils::*;

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);

    let q = rel.query().limit(1);
    check_query_vec(&rel, q, "Vec[it:Int; 1]");

    let q = rel.query().skip(1);
    check_query_vec(&rel, q, "Vec[it:Int; 2; 3]");

    let q = rel.query().skip(1).limit(1);
    check_query_vec(&rel, q, "Vec[it:Int; 2]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);

    let q = rel.query().limit(1);
    check_query_tree(&rel, q, "Tree[pk key:Int, value:Int; 1, 2]");

    let q = rel.query().skip(1);
    check_query_tree(&rel, q, "Tree[pk key:Int, value:Int; 3, 4; 5, 6]");

    let q = rel.query().skip(1).limit(1);
    check_query_tree(&rel, q, "Tree[pk key:Int, value:Int; 3, 4]");
}
