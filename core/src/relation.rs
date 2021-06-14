use crate::for_impl::*;
use crate::prelude::*;

use derive_more::From;
use dyn_clone::DynClone;

pub trait Rel: fmt::Debug + DynClone {
    fn type_name(&self) -> &str;

    fn kind(&self) -> DataType;

    fn schema(&self) -> Schema;

    fn len(&self) -> usize;
    fn size(&self) -> ShapeLen;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn is_scalar(&self) -> bool {
        self.size().is_scalar()
    }

    fn as_any(&self) -> &dyn Any;

    fn rel_hash(&self, hasher: &mut dyn Hasher);
    fn rel_eq(&self, other: &dyn Rel) -> bool;
    fn rel_cmp(&self, other: &dyn Rel) -> Ordering;

    fn as_i64(&self) -> Option<ScalarNative<i64>> {
        None
    }
    fn as_string(&self) -> Option<ScalarNative<Rc<String>>> {
        None
    }

    fn iter(&self) -> Box<IterScalar<'_>>;
    fn col(&self, pos: usize) -> Col<'_>;
    fn rows(&self) -> Box<IterRows<'_>>;

    fn query(&self) -> QueryOp {
        QueryOp::new(self.schema())
    }
}

#[derive(Debug, From)]
pub struct RelationDyn {
    pub rel: Box<dyn Rel>,
}

impl Clone for RelationDyn {
    fn clone(&self) -> Self {
        RelationDyn {
            rel: dyn_clone::clone_box(&*self.rel),
        }
    }
}

impl PartialEq for RelationDyn {
    fn eq(&self, other: &Self) -> bool {
        self.rel.rel_eq(&*other.rel)
    }
}
impl Eq for RelationDyn {}

impl PartialOrd for RelationDyn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rel.rel_cmp(&*other.rel))
    }
}

impl Ord for RelationDyn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rel.rel_cmp(&*other.rel)
    }
}

impl fmt::Display for RelationDyn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rel.type_name())?;
        write!(f, "{}", self.rel.schema())?;
        Ok(())
    }
}

pub trait ToHash {
    fn to_hash(&self, h: &mut dyn Hasher);
}

impl ToHash for dyn Rel {
    fn to_hash(&self, h: &mut dyn Hasher) {
        self.rel_hash(h)
    }
}

impl Hash for RelationDyn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rel.rel_hash(state);
    }
}
