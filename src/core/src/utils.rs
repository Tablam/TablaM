use std::fmt;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Postfixer<'a, T> {
    val: &'a T,
    fix: &'a str,
}

impl<T: fmt::Display> fmt::Display for Postfixer<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.val, self.fix)
    }
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

pub fn format_slice_bit(list: &[bool], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_list(
        list.iter().map(|x| if *x { '1' } else { '0' }),
        list.len(),
        "Bits[",
        "b]",
        f,
    )
}

pub fn format_slice_scalar<I>(list: &[I], f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    I: fmt::Display,
{
    if list.len() == 1 {
        write!(f, "{}", list[0])
    } else {
        format_list(list, list.len(), "[", "]", f)
    }
}

pub fn format_slice_scalar_postfix<I>(
    list: &[I],
    fix: &str,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    I: fmt::Display,
{
    if list.len() == 1 {
        let x = Postfixer { val: &list[0], fix };
        write!(f, "{}", x)
    } else {
        format_list(
            list.iter().map(|val| Postfixer { val, fix }),
            list.len(),
            "[",
            "]",
            f,
        )
    }
}
