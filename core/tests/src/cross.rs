use tablam::prelude::*;
use tablam::query::JoinOp;

#[test]
fn test_vec() {
    let lhs = array(&[1, 2, 3]);
    let rhs = array(&["a", "b", "c"]);
    let q = JoinOp::cross(lhs.schema.clone(), rhs.schema.clone());
    let q = q.execute(lhs.rows_iter(), rhs.rows_iter());

    let rel = Vector::from_iter(q.schema, q.iter);
    assert_eq!(
        &format!("{}", rel),
        "Vec[I64; 1, 'a'; 1, 'b'; 1, 'c'; 2, 'a'; 2, 'b'; 2, 'c'; 3, 'a'; 3, 'b'; 3, 'c']"
    );
}
