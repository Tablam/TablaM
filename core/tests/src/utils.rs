use tablam::prelude::*;

pub fn check_join_vec(lhs: &Vector, rhs: &Vector, q: JoinOp, result: &str) {
    let q = q.execute(lhs.rows(), rhs.rows());
    let rows = q.iter.map(|x| x.to_vec()).flatten().collect();
    let rel = Vector::new_table(rows, q.schema);
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_query_vec(rel: &Vector, q: QueryOp, result: &str) {
    let q = q.execute(rel.rows());
    let rows = q.iter.map(|x| x.to_vec()).flatten().collect();
    let rel = Vector::new_table(rows, q.schema);
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_join_tree(lhs: &Tree, rhs: &Tree, q: JoinOp, result: &str) {
    let q = q.execute(lhs.rows(), rhs.rows());
    let rel = Tree::from_iter(q.schema, q.iter.map(|x| x.to_vec()));
    assert_eq!(&format!("{}", rel), result);
}

pub fn check_query_tree(rel: &Tree, q: QueryOp, result: &str) {
    let q = q.execute(rel.rows());
    let rel = Tree::from_iter(q.schema, q.iter.map(|x| x.to_vec()));
    assert_eq!(&format!("{}", rel), result);
}
