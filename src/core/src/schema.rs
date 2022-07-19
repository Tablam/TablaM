//! # Relational schema.
//!
//! A relational schema specifies the set of [Field] (attributes) in the inner container and a [DataType] for each field,
//! and gives the guarantee that 2 schemas are equal if (irrespective of the *order* of the fields), both match.

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::prelude::*;

/// The default field/column name for [Scalar]/[Vector] relations
const FIELD_NAME_SCALAR: &str = "it";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldSlice<'a> {
    pub name: &'a str,
    pub kind: DataType,
}

impl<'a> FieldSlice<'a> {
    pub fn new(name: &'a str, kind: DataType) -> Self {
        Self { name, kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: String,
    pub kind: DataType,
}

impl<'a> From<&'a Field> for FieldSlice<'a> {
    fn from(x: &'a Field) -> Self {
        FieldSlice {
            name: &x.name,
            kind: x.kind.clone(),
        }
    }
}

impl Field {
    pub fn new(name: &str, kind: DataType) -> Self {
        Field {
            name: name.to_string(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Schema {
    pub pk: Option<usize>,
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(pk: Option<usize>, fields: &[Field]) -> Self {
        if !fields.is_empty() {
            assert!(
                pk.unwrap_or(0) < fields.len(),
                "The selected PK is out of bounds"
            );
        }
        Self {
            pk,
            fields: fields.into(),
        }
    }

    pub fn new_single(name: &str, kind: DataType) -> Self {
        let field = Field::new(name, kind);
        Self::new(Some(0), &[field])
    }

    pub fn new_scalar(kind: DataType) -> Self {
        let field = Field::new(FIELD_NAME_SCALAR, kind);
        Self::new(Some(0), &[field])
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

impl PartialEq for Schema {
    fn eq(&self, other: &Schema) -> bool {
        if self.fields.len() == other.fields.len() {
            let mut a = self.fields.clone();
            let mut b = other.fields.clone();
            a.sort();
            b.sort();
            a == b
        } else {
            false
        }
    }
}

impl Eq for Schema {}

impl Hash for Schema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut a = self.fields.clone();
        a.sort();
        a.hash(state);
    }
}

pub enum SchemaInfo {
    Scalar(DataType),
    Vec(Rc<Schema>),
}

impl SchemaInfo {
    pub(crate) fn scalar(of: DataType) -> Self {
        Self::Scalar(of)
    }

    pub(crate) fn vec(of: &Rc<Schema>) -> Self {
        Self::Vec(of.clone())
    }

    fn schema(&self) -> Schema {
        unimplemented!()
    }

    fn kind(&self) -> DataType {
        unimplemented!()
    }

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, pos: usize) -> Option<&FieldSlice<'_>> {
        unimplemented!()
    }

    fn named(&self, name: &str) -> Option<&FieldSlice<'_>> {
        unimplemented!()
    }
    fn pk(&self) -> Option<usize> {
        unimplemented!()
    }
    fn pk_field(&self) -> Option<&FieldSlice<'_>> {
        if let Some(pos) = self.pk() {
            self.get(pos)
        } else {
            None
        }
    }
}
