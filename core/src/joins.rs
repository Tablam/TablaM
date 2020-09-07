use std::collections::HashSet;

use derive_more::Display;
use genawaiter::rc::Gen;

use crate::prelude::*;
use crate::scalar::combine;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display)]
pub enum Join {
    #[display(fmt = "&cross")]
    Cross,
    #[display(fmt = "&left")]
    Left,
    #[display(fmt = "&right")]
    Right,
    #[display(fmt = "&inner")]
    Inner,
    #[display(fmt = "&full")]
    Full,
    #[display(fmt = "&")]
    Natural,
}

impl Join {
    pub fn produce_null(self, is_left: bool) -> bool {
        match self {
            Join::Left => !is_left,
            Join::Right => is_left,
            Join::Inner => false,
            Join::Cross => false,
            Join::Full => true,
            Join::Natural => false,
        }
    }
}

pub fn cross<'a, 'b>(
    lhs: impl Iterator<Item = Tuple> + 'a,
    rhs: impl Iterator<Item = Tuple> + 'b,
) -> impl Iterator<Item = Tuple> {
    Gen::new(|co| async move {
        let rhs: Vec<_> = rhs.collect();
        for a in lhs {
            for b in &rhs {
                // dbg!(&a, b);

                co.yield_(combine(&a, &b)).await;
            }
        }
    })
    .into_iter()
}

pub fn left_join<'a, 'b>(
    lhs: impl Iterator<Item = Tuple> + 'a,
    rhs: impl Iterator<Item = Tuple> + 'b,
    fields_rhs: usize,
) -> impl Iterator<Item = Tuple> {
    Gen::new(|co| async move {
        let rhs: HashSet<_> = rhs.collect();
        for a in lhs {
            if let Some(b) = rhs.get(&a) {
                co.yield_(combine(&a, &b)).await;
            } else {
                co.yield_(combine(&a, &Scalar::Unit.repeat(fields_rhs)))
                    .await;
            }
        }
    })
    .into_iter()
}

pub fn difference<'a, 'b>(
    lhs: impl Iterator<Item = Tuple> + 'a,
    rhs: impl Iterator<Item = Tuple> + 'b,
) -> impl Iterator<Item = Tuple> {
    Gen::new(|co| async move {
        let rhs: HashSet<_> = rhs.collect();
        for a in lhs {
            if !rhs.contains(&a) {
                co.yield_(a).await;
            }
        }
    })
    .into_iter()
}

pub fn intersect<'a, 'b>(
    lhs: impl Iterator<Item = Tuple> + 'a,
    rhs: impl Iterator<Item = Tuple> + 'b,
) -> impl Iterator<Item = Tuple> {
    Gen::new(|co| async move {
        let rhs: HashSet<_> = rhs.collect();
        for a in lhs {
            if rhs.contains(&a) {
                co.yield_(a).await;
            }
        }
    })
    .into_iter()
}
