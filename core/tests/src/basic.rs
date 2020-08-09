use tablam::prelude::*;

#[test]
fn test_mem_size() {
    //Just a sanity check to not blow up the memory!
    assert_eq!(std::mem::size_of::<Scalar>(), 24);
}
