use tablam::dsl::{field, value};
use tablam::scalar::{Col, Scalar};
use tablam::schema::Schema;
use tablam::types::*;

pub fn nums_1() -> Vec<i64> {
    vec![1, 2, 3]
}
pub fn nums_2() -> Vec<i64> {
    vec![4, 5, 6]
}
pub fn nums_3() -> Vec<i64> {
    vec![2, 3, 4]
}
pub fn bools_1() -> Vec<bool> {
    vec![true, false, true]
}
pub fn bools_2() -> Vec<bool> {
    vec![false, true, false]
}

pub fn schema1() -> Schema {
    Schema::new(
        [
            field("one", DataType::I64),
            field("two", DataType::I64),
            field("three", DataType::Bool),
        ]
        .to_vec(),
    )
}

fn schema_cross() -> Schema {
    Schema::new([field("a", DataType::I64), field("b", DataType::I64)].to_vec())
}
