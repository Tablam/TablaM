use tablam::prelude::*;

pub fn check_join_vec(lhs: &Vector, rhs: &Vector, q: JoinOp, result: &str) {
    let q = q.execute(lhs.rows_iter(), rhs.rows_iter());
    let rel = Vector::from_iter(q.schema, q.iter);
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_query_vec(rel: &Vector, q: QueryOp, result: &str) {
    let q = q.execute(rel.rows_iter());
    let rel = Vector::from_iter(q.schema, q.iter);
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_join_tree(lhs: &Tree, rhs: &Tree, q: JoinOp, result: &str) {
    let q = q.execute(lhs.rows_iter(), rhs.rows_iter());
    let rel = Tree::from_iter(q.schema, q.iter);
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_query_tree(rel: &Tree, q: QueryOp, result: &str) {
    let q = q.execute(rel.rows_iter());
    let rel = Tree::from_iter(q.schema, q.iter);
    assert_eq!(&format!("{}", rel), result);
}
