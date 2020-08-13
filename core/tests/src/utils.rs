use tablam::dsl::field;
use tablam::schema::Schema;
use tablam::types::*;

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
