use tablam::prelude::*;
use tablam::stdlib::math::math_add;
use tablam::stdlib::math::math_fn;

#[test]
fn higer_order() {
    // Verify we can use functions as anything else
    let f = math_fn("add", DataType::I64, math_add);
    let x: Scalar = f.into();
    assert_eq!(format!("{}", &x), String::from("Fun()"));
    assert_eq!(
        format!("{}", &x.kind()),
        String::from("Fun((left: Int, right: Int)= Int)")
    );

    let rel = scalar(x);
    assert_eq!(
        &format!("{}", rel),
        "Vec[it:Fun((left: Int, right: Int)= Int); Fun()]"
    );
}
