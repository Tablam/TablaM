/// Macros and Utilities for making conversions from/to TablaM types
use crate::prelude::*;

macro_rules! convert {
    ($kind:ident, $bound:path) => {
        impl<'a> From<&'a $kind> for Scalar {
            fn from(i: &'a $kind) -> Self {
                $bound([i.clone()])
            }
        }

        impl<'a> From<&'a [$kind; 1]> for Scalar {
            fn from(i: &'a [$kind; 1]) -> Self {
                $bound(i.clone())
            }
        }

        impl From<[$kind; 1]> for Scalar {
            fn from(i: [$kind; 1]) -> Self {
                $bound(i)
            }
        }

        impl From<$kind> for Scalar {
            fn from(i: $kind) -> Self {
                $bound([i])
            }
        }

        impl From<Option<$kind>> for Scalar {
            fn from(i: Option<$kind>) -> Self {
                match i {
                    Some(x) => $bound([x]),
                    _ => unreachable!("Option {:?}", i),
                }
            }
        }

        impl<'a> From<&'a Scalar> for &'a [$kind] {
            fn from(i: &'a Scalar) -> Self {
                match i {
                    $bound(x) => x,
                    _ => unreachable!("Slice {:?}", i),
                }
            }
        }

        impl From<Scalar> for $kind {
            fn from(i: Scalar) -> Self {
                match i {
                    $bound(x) => x[0].clone(),
                    _ => unreachable!("Scalar {:?}", i),
                }
            }
        }

        impl<'a> From<&'a Scalar> for Option<$kind> {
            fn from(i: &'a Scalar) -> Self {
                match i {
                    $bound(x) => Some(x[0].clone()),
                    _ => None,
                }
            }
        }

        impl<'a> From<&'a Scalar> for Option<&'a $kind> {
            fn from(i: &'a Scalar) -> Self {
                match i {
                    $bound(x) => Some(&x[0]),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! convert_vec {
    ($kind:ident, $bound:path) => {
        impl<'a> From<&'a $kind> for Array {
            fn from(i: &'a $kind) -> Self {
                $bound(vec![i.clone()])
            }
        }

        impl<'a> From<&'a [$kind]> for Array {
            fn from(i: &'a [$kind]) -> Self {
                $bound(i.into())
            }
        }

        impl From<$kind> for Array {
            fn from(i: $kind) -> Self {
                $bound(vec![i])
            }
        }

        impl From<Option<$kind>> for Array {
            fn from(i: Option<$kind>) -> Self {
                match i {
                    Some(x) => $bound(vec![x]),
                    _ => unreachable!("Option {:?}", i),
                }
            }
        }

        impl From<Array> for Vec<$kind> {
            fn from(i: Array) -> Self {
                match i {
                    $bound(x) => x,
                    _ => unreachable!("Array {:?}", i),
                }
            }
        }
    };
}

macro_rules! kind_native {
    ($native:ident, $kind:ident, $num_rows:literal) => {
        impl NativeKind for $native {
            fn kind() -> DataType {
                DataType::$kind
            }
            fn num_rows() -> usize {
                $num_rows
            }
        }
    };
}

kind_native!(i64, I64, 1);
kind_native!(bool, Bool, 1);
kind_native!(Decimal, Decimal, 1);
kind_native!(F64, F64, 1);
kind_native!(f64, F64, 1);
kind_native!(String, Utf8, 1);

impl From<&[bool]> for Array {
    fn from(x: &[bool]) -> Self {
        Array::Bool(x.into())
    }
}

impl From<bool> for Scalar {
    fn from(x: bool) -> Self {
        Scalar::Bool([x])
    }
}

impl From<&str> for Scalar {
    fn from(x: &str) -> Self {
        Scalar::Utf8([x.into()])
    }
}

impl From<DateT> for Scalar {
    fn from(x: DateT) -> Self {
        Scalar::Date([x])
    }
}

impl From<&str> for Array {
    fn from(x: &str) -> Self {
        Array::Utf8(vec![x.into()])
    }
}

impl From<&[&str]> for Array {
    fn from(x: &[&str]) -> Self {
        Array::Utf8(x.iter().map(|x| x.to_string()).collect())
    }
}

convert!(i64, Scalar::I64);
convert_vec!(i64, Array::I64);
convert!(F64, Scalar::F64);
convert_vec!(F64, Array::F64);
convert!(Decimal, Scalar::Decimal);
convert_vec!(Decimal, Array::Decimal);
convert!(String, Scalar::Utf8);
convert_vec!(String, Array::Utf8);
