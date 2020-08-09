use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Index;

use bit_vec::BitVec;

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: String,
    pub kind: DataType,
}

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Schema {
    pub columns: Vec<Field>,
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

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        Schema { columns: fields }
    }

    pub fn new_single(name: &str, kind: DataType) -> Self {
        let field = Field::new(name, kind);
        Self::new(vec![field])
    }

    pub fn scalar_field(kind: DataType) -> Self {
        Self::new_single("it", kind)
    }

    pub fn generate(types: &[DataType]) -> Self {
        let mut names = Vec::with_capacity(types.len());

        for (pos, kind) in types.iter().enumerate() {
            names.push(Field::new_owned(pos.to_string(), kind.clone()));
        }

        Self::new(names)
    }

    pub fn named(&self, name: &str) -> Option<(usize, &Field)> {
        self.columns
            .iter()
            .enumerate()
            .find(|&(_, field)| field.name == name)
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> Vec<&str> {
        self.columns.iter().map(|x| x.name.as_ref()).collect()
    }

    ///Recover the column position from the relative ColumnName
    pub fn resolve_pos(&self, of: &Column) -> usize {
        match of {
            Column::Pos(x) => *x,
            Column::Name(x) => {
                let (pos, _f) = self.named(x).unwrap();
                pos
            }
        }
    }

    pub fn resolve_pos_many(&self, of: &[Column]) -> Pos {
        of.iter().map(|x| self.resolve_pos(x)).collect()
    }

    ///Recover the column names from a list of relative ColumnName
    pub fn resolve_names(&self, of: &[Column]) -> Schema {
        let mut names = Vec::with_capacity(of.len());

        for name in of.iter() {
            let pick = match name {
                Column::Pos(x) => self.columns[*x].clone(),
                Column::Name(x) => {
                    let (_pos, f) = self.named(x).unwrap();
                    f.clone()
                }
            };
            names.push(pick);
        }
        Self::new(names)
    }

    pub fn join(&self, other: &Self) -> Vec<usize> {
        let mut fields = Vec::new();
        for (i, col) in other.columns.iter().enumerate() {
            if self.exist(&col.name) {
                continue;
            } else {
                fields.push(i);
            }
        }

        fields
    }

    /// Helper for select/projection
    pub fn only(&self, position: &[usize]) -> Self {
        let mut fields = Vec::with_capacity(position.len());
        for pos in position {
            fields.push(self.columns[*pos].clone());
        }
        Self::new(fields)
    }

    pub fn except(&self, remove: &[usize]) -> Pos {
        let mut all = BitVec::from_elem(self.len(), true);
        let mut pos = Vec::with_capacity(self.len());

        for i in remove {
            all.set(*i, false);
        }

        for (i, ok) in all.iter().enumerate() {
            if ok {
                pos.push(i);
            }
        }
        pos
    }

    pub fn deselect(&self, remove: &[usize]) -> Self {
        let deselect = self.except(remove);
        self.only(deselect.as_slice())
    }

    pub fn exist(&self, field: &str) -> bool {
        let mut find = self.columns.iter().filter(|x| x.name == field);

        find.next().is_some()
    }

    pub fn extend(&self, right: &Schema) -> Self {
        let count = self.len() + right.len();
        let mut fields = Vec::with_capacity(count);
        let mut left = self.columns.clone();
        let mut _right = right.columns.clone();

        fields.append(&mut left);
        let mut cont = 0;
        //Avoid duplicated field names...
        for f in _right {
            if self.exist(&f.name) {
                let name = format!("{}_{}", f.name, cont);
                fields.push(Field::new(&name, f.kind));
                cont += 1;
            } else {
                fields.push(f);
            }
        }

        Self::new(fields)
    }

    pub fn rename(&self, change: &[ColumnAlias]) -> Self {
        let mut names = self.columns.clone();

        for col in change {
            let pos = self.resolve_pos(&col.from);
            let old = names[pos].kind.clone();
            names[pos] = Field::new(&col.to, old);
        }

        Self::new(names)
    }

    pub fn project(&self, select: &ProjectDef) -> Pos {
        match select {
            ProjectDef::Select(pos) => self.resolve_pos_many(&pos),
            ProjectDef::Deselect(pos) => self.except(&self.resolve_pos_many(&pos)),
        }
    }

    pub fn kind(&self) -> DataType {
        DataType::Table(self.columns.iter().map(|x| x.kind.clone()).collect())
    }
}

impl Index<usize> for Schema {
    type Output = Field;

    fn index(&self, pos: usize) -> &Field {
        &self.columns[pos]
    }
}

impl PartialEq for Schema {
    fn eq(&self, other: &Schema) -> bool {
        if self.columns.len() == other.columns.len() {
            let mut a = self.columns.clone();
            let mut b = other.columns.clone();
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
        let mut a = self.columns.clone();
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
            let item = &self.columns[i];
            if i > 0 {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
        }

        Ok(())
    }
}
