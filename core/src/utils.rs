use std::cmp::Ordering;
use std::fmt;
use std::sync::Mutex;

use crate::prelude::{Rel, Row, Schema};
use crate::scalar::Scalar;
use crate::types::{ScalarNative, ShapeLen};

use slotmap::{DefaultKey, SlotMap};

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library version.
pub fn version() -> String {
    VERSION.to_string()
}

pub fn is_t<T: 'static>(of: &dyn Rel) -> bool {
    std::any::TypeId::of::<T>() == of.as_any().type_id()
}

pub fn cmp_eq<T: 'static>(of: &T, other: &dyn Rel) -> bool
where
    T: PartialEq + fmt::Debug,
{
    //dbg!(&of.type_id(), &other.as_any().type_id());
    let y = other.as_any();

    if let Some(x) = y.downcast_ref::<T>() {
        of == x
    } else {
        false
    }
}

pub fn cmp<T: 'static>(of: &T, other: &dyn Rel) -> Ordering
where
    T: Ord + Rel,
{
    if let Some(x) = other.as_any().downcast_ref::<T>() {
        of.cmp(&x)
    } else {
        of.schema().cmp(&other.schema())
    }
}

lazy_static! {
    static ref SYMBOLS: Mutex<SlotMap<slotmap::DefaultKey, String>> = {
        let mut m = SlotMap::with_capacity(1024);
        m.insert("one".into());
        Mutex::new(m)
    };
}

pub fn intern(key: &str) -> DefaultKey {
    SYMBOLS.lock().unwrap().insert(key.into())
}

pub fn format_list<I>(
    list: impl IntoIterator<Item = I>,
    total: usize,
    start: &str,
    end: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    I: fmt::Display,
{
    write!(f, "{}", start)?;

    for (pos, x) in list.into_iter().enumerate() {
        if pos < total - 1 {
            write!(f, "{}, ", x)?;
        } else {
            write!(f, "{}", x)?;
        }
    }

    write!(f, "{}", end)
}

pub fn fmt_table<'a>(
    name: &str,
    schema: &Schema,
    size: ShapeLen,
    iter: impl Iterator<Item = Row<'a>>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if size.cols() > 0 {
        write!(f, "{}[{}", name, schema)?;
        let total = size.rows().unwrap_or_default();
        if total > 0 {
            write!(f, ";")?;
        }
        for (row_pos, row) in iter.enumerate() {
            if row_pos < total - 1 {
                write!(f, "{};", row)?;
            } else {
                write!(f, "{}", row)?;
            }
        }
        write!(f, "]")
    } else {
        write!(f, "{}[]", name)
    }
}

pub fn fmt_row<'a>(
    iter: impl Iterator<Item = &'a Scalar>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut exist = false;
    for x in iter {
        if exist {
            write!(f, ",")?;
        }
        exist = true;
        write!(f, " {}", x)?;
    }

    Ok(())
}

pub trait Ops {
    fn bin_op_i64<Op>(op: Op, x: Scalar, y: Scalar) -> Scalar
    where
        Op: Fn(i64, i64) -> i64,
    {
        match (
            x.as_i64().expect("x is not i64"),
            y.as_i64().expect("y is not i64"),
        ) {
            (ScalarNative::One(a), ScalarNative::One(b)) => op(*a, *b).into(),
            _ => unreachable!(),
        }
    }
}

pub fn bin_op<T, Op>(op: Op, x: T, y: T) -> Scalar
where
    Op: Fn(T, T) -> T,
    T: Into<Scalar>,
    Scalar: From<T>,
{
    op(x, y).into()
}
