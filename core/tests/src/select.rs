use tablam::prelude::*;

use crate::utils::*;

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);

    let q = rel.query().select(&[]);
    check_query_vec(&rel, q, "Vec[]");

    let q = rel.query().select(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[Int; 1; 2; 3]");

    let rel = narray(3, [1, 2, 3, 4, 5, 6].chunks(3));

    let q = rel.query().select(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[Int; 1; 4]");

    let q = rel.query().deselect(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[Int, Int; 2, 3; 5, 6]");
}

#[test]
fn test_tree() {
    let rel = tree_kv(&[1, 2, 3, 4, 5, 6]);

    let q = rel.query().select(&[]);
    check_query_tree(&rel, q, "Tree[]");

    let q = rel.query().select(&[colp(0)]);
    check_query_tree(&rel, q, "Tree[pk key:Int; 1; 3; 5]");

    let q = rel.query().deselect(&[colp(0)]);
    check_query_tree(&rel, q, "Tree[pk value:Int; 2; 4; 6]");
}
