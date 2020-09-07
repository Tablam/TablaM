use crate::utils::*;
use tablam::prelude::*;

#[test]
fn test_vec() {
    let lhs = array(&[1, 2, 3]);
    let rhs = array(&["a", "b", "c"]);
    let q = JoinOp::join_left(lhs.schema.clone(), rhs.schema.clone());

    check_join_vec(
        &lhs,
        &rhs,
        q,
        "Vec[it:Int, it_2:Str; 1, Unit; 2, Unit; 3, Unit]",
    );

    let rhs = array(&[1, 2, 3]);
    let q = JoinOp::join_left(lhs.schema.clone(), rhs.schema.clone());

    check_join_vec(&lhs, &rhs, q, "Vec[it:Int, it_2:Int; 1, 1; 2, 2; 3, 3]");
}

#[test]
fn test_tree() {
    let lhs = tree_kv2(&[(1, "a"), (2, "b")]);
    let rhs = tree_kv2(&[(4.0, true), (5.0, false)]);

    let q = JoinOp::join_left(lhs.schema.clone(), rhs.schema.clone());

    check_join_tree(&lhs, &rhs, q, "Tree[pk key:Int, value:Str, key_2:Float, value_2:Bool; 1, 'a', Unit, Unit; 2, 'b', Unit, Unit]");

    let rhs = tree_kv2(&[(1, "a"), (2, "b")]);

    let q = JoinOp::join_left(lhs.schema.clone(), rhs.schema.clone());

    check_join_tree(
        &lhs,
        &rhs,
        q,
        "Tree[pk key:Int, value:Str, key_2:Int, value_2:Str; 1, 'a', 1, 'a'; 2, 'b', 2, 'b']",
    );
}
