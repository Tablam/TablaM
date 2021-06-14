use tablam::prelude::*;

use crate::utils::*;

#[test]
fn test_vec() {
    let rel = array(&[1, 2, 3]);

    let q = rel.query().select(&[]);
    check_query_vec(&rel, q, "Vec[]");

    let q = rel.query().select(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[it:Int; 1; 2; 3]");

    let rel = narray(3, [1, 2, 3, 4, 5, 6].iter());

    let q = rel.query().select(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[col_0:Int; 1; 4]");

    let q = rel.query().deselect(&[colp(0)]);
    check_query_vec(&rel, q, "Vec[col_1:Int, col_2:Int; 2, 3; 5, 6]");
}

#[test]
fn test_rename() {
    let rel = narray(3, [1, 2, 3, 4, 5, 6].iter());

    let q = rel.query().select(&[colp_as(0, "one")]);
    check_query_vec(&rel, q, "Vec[one:Int; 1; 4]");

    let rel = tree_kv(&[1, 2]);
    let q = rel.query().select(&[colp_as(0, "one")]);
    check_query_tree(&rel, q, "Tree[pk one:Int; 1]");
}

#[test]
fn test_names() {
    let rel = array(&[1, 2, 3]);

    let q = rel.query().select(&[coln("it")]);
    check_query_vec(&rel, q, "Vec[it:Int; 1; 2; 3]");
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
//
// #[test]
// fn test_file() {
//     let mut temp = std::env::temp_dir().to_path_buf();
//     temp.push("sample.csv");
//     //Hack around the fact file support is broken...
//
//     let mut f = std::fs::File::create(&temp).unwrap();
//     f.write_all(PRODUCTS_CSV.as_ref()).unwrap();
//     drop(f);
//     let rel = File::new(temp, true, true, true).unwrap();
//     // rel.write_string(PRODUCTS_CSV).unwrap();
//     // rel.seek_start(0).unwrap();
//
//     let q = rel.query().select(&[colp(0)]).limit(1);
//     let q = q.execute(rel.rows_iter());
//
//     let v = Vector::from_iter(q.schema, q.iter);
//     assert_eq!(&format!("{}", v), "Vec[id:Str; '1']");
// }
