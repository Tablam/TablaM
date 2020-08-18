use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Index;

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: String,
    pub kind: DataType,
}

impl Field {
    pub fn new(name: &str, kind: DataType) -> Self {
        Field {
            name: name.to_string(),
            kind,
        }
    }

    pub fn new_owned(name: String, kind: DataType) -> Self {
        Field { name, kind }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn kind(&self) -> &DataType {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Schema {
    pub pk: Option<usize>,
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(fields: Vec<Field>, pk: Option<usize>) -> Self {
        Schema { pk, fields }
    }

    pub fn new_single(name: &str, kind: DataType) -> Self {
        let field = Field::new(name, kind);
        Self::new(vec![field], None)
    }

    pub fn scalar_field(kind: DataType) -> Self {
        Self::new_single("it", kind)
    }

    pub fn named(&self, name: &str) -> Option<(usize, &Field)> {
        self.fields
            .iter()
            .enumerate()
            .find(|&(_, field)| field.name == name)
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pk_field(&self) -> Option<Field> {
        if let Some(pos) = self.pk {
            self.fields
                .get(self.resolve_pos(&Column::Pos(pos)))
                .cloned()
        } else {
            None
        }
    }

    pub fn resolve_name(&self, of: &Column) -> (usize, Field) {
        match of {
            Column::Pos(x) => (*x, self.fields[*x].clone()),
            Column::Name(x) => {
                let (pos, f) = self.named(x).unwrap();
                (pos, f.clone())
            }
            Column::Alias(x) => {
                let (pos, mut f) = self.resolve_name(&x.from);
                f.name = x.to.clone();
                (pos, f)
            }
        }
    }

    ///Recover the column position from the relative ColumnName
    pub fn resolve_pos(&self, of: &Column) -> usize {
        match of {
            Column::Pos(x) => *x,
            Column::Name(x) => {
                let (pos, _f) = self.named(x).unwrap();
                pos
            }
            Column::Alias(x) => self.resolve_pos(&x.from),
        }
    }

    pub fn resolve_pos_many(&self, of: &[Column]) -> Pos {
        of.iter().map(|x| self.resolve_pos(x)).collect()
    }

    pub fn pick_new_pk(&mut self, old: Option<Field>) {
        if let Some(pk) = old {
            if let Some((pos, _)) = self.named(&pk.name) {
                self.pk = Some(pos);
            } else {
                self.pk = Some(0);
            }
        }
    }

    pub fn extend(&self, right: &Schema) -> Self {
        let mut fields = Vec::with_capacity(self.len() + right.len());
        fields.append(&mut self.fields.clone());

        let mut find: HashMap<String, usize> =
            self.fields.iter().map(|x| (x.name.clone(), 2)).collect();

        //Avoid duplicated field names...
        for f in right.fields.clone() {
            if find.contains_key(&f.name) {
                let cont = find[&f.name];
                find.insert(f.name.clone(), cont + 1);

                let name = format!("{}_{}", f.name, cont);
                fields.push(Field::new(&name, f.kind));
            } else {
                fields.push(f);
            }
        }

        Self::new(fields, self.pk)
    }

    pub fn project(&self, select: &ProjectDef) -> (Schema, Pos) {
        let pk = self.pk_field();
        let mut selected: Vec<Field> = Vec::new();
        let mut pos = Vec::new();
        let resolved = select.columns().iter().map(|f| self.resolve_name(f));
        let total = select.columns().len();
        let mut to_select = HashSet::with_capacity(total);
        let mut fields = Vec::with_capacity(total);

        for (pos, f) in resolved {
            to_select.insert(pos);
            fields.push(f);
        }

        match select {
            ProjectDef::Select(_) => {
                for (i, _) in self.fields.iter().enumerate() {
                    if to_select.contains(&i) {
                        selected.push(fields[i].clone());
                        pos.push(i);
                    }
                }
            }
            ProjectDef::Deselect(_) => {
                for (i, f) in self.fields.iter().enumerate() {
                    if !to_select.contains(&i) {
                        selected.push(f.clone());
                        pos.push(i);
                    }
                }
            }
        };

        let mut schema = Schema::new(selected, None);
        schema.pick_new_pk(pk);
        (schema, pos)
    }

    pub fn kind(&self) -> Vec<DataType> {
        self.fields.iter().map(|x| x.kind.clone()).collect()
    }
}

pub(crate) fn check_pk(schema: &Schema) -> usize {
    schema.pk.expect("Relation need a pk")
}

impl Index<usize> for Schema {
    type Output = Field;

    fn index(&self, pos: usize) -> &Field {
        &self.fields[pos]
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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.kind)
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len() {
            let item = &self.fields[i];
            if Some(i) == self.pk {
                if i > 0 {
                    write!(f, ", pk {}", item)?;
                } else {
                    write!(f, "pk {}", item)?;
                }
            } else if i > 0 {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
        }

        Ok(())
    }
}
