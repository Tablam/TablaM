use crate::utils::*;
use tablam::prelude::*;

#[test]
fn test_vec() {
    let lhs = array(&[1, 2, 3]);
    let rhs = array(&["a", "b", "c"]);
    let q = JoinOp::cross(lhs.schema.clone(), rhs.schema.clone());

    check_join_vec(
        &lhs,
        &rhs,
        q,
        "Vec[Int, Str; 1, 'a'; 1, 'b'; 1, 'c'; 2, 'a'; 2, 'b'; 2, 'c'; 3, 'a'; 3, 'b'; 3, 'c']",
    );
}

#[test]
fn test_tree() {
    let lhs = tree_kv2(&[(1, "a"), (2, "b")]);
    let rhs = tree_kv2(&[(4.0, true), (5.0, false)]);

    let q = JoinOp::cross(lhs.schema.clone(), rhs.schema.clone());

    check_join_tree(&lhs, &rhs, q, "Tree[pk key:Int, value:Str, key_2:Float, value_2:Bool; 1, 'a', 4, true; 1, 'a', 5, false; 2, 'b', 4, true; 2, 'b', 5, false]");
}
