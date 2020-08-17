use crate::utils::*;
use tablam::prelude::*;

#[test]
fn test_vec() {
    let lhs = array(&[1, 2, 3]);
    let rhs = array(&[4, 5, 6]);
    let q = JoinOp::union(lhs.schema.clone(), rhs.schema.clone()).unwrap();

    check_join_vec(&lhs, &rhs, q, "Vec[Int; 1; 2; 3; 4; 5; 6]");
}

#[test]
fn test_tree() {
    let lhs = tree_kv2(&[(1, "a"), (2, "b")]);
    let rhs = tree_kv2(&[(3, "c"), (4, "d")]);

    let q = JoinOp::union(lhs.schema.clone(), rhs.schema.clone()).unwrap();

    check_join_tree(
        &lhs,
        &rhs,
        q,
        "Tree[pk key:Int, value:Str; 1, 'a'; 2, 'b'; 3, 'c'; 4, 'd']",
    );
}
