use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::PathBuf;

use console::Term;
use derive_more::{Display, From};

use crate::for_impl::*;
use crate::prelude::*;

pub struct Console {
    _out: Term,
    _err: Term,
}

impl Console {
    pub fn new() -> Self {
        Console {
            _out: Term::buffered_stdout(),
            _err: Term::buffered_stderr(),
        }
    }

    pub fn write_line(&self, of: &str) -> Result<()> {
        self._out.write_line(of)?;
        Ok(())
    }

    pub fn write_str(&self, of: &str) -> Result<()> {
        self._out.write_str(of)?;
        Ok(())
    }
}

impl Default for io::Console {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Display, Derivative, From)]
#[display(fmt = "File([{}], {:?})", schema, path)]
#[derivative(Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct File {
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    f: fs::File,
    schema: Schema,
    path: PathBuf,
    read: bool,
    write: bool,
    create: bool,
}

impl File {
    pub fn map_err<T>(of: std::io::Result<T>, path: &PathBuf) -> Result<T> {
        of.map_err(|e| Error::file_err(e, path.clone()))
    }

    pub fn new(path: PathBuf, read: bool, write: bool, create: bool) -> Result<Self> {
        let f = fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(create)
            .open(&path);

        let f = Self::map_err(f, &path)?;
        let schema = if let Some(header) = BufReader::new(&f).lines().next() {
            match header {
                Ok(header) => {
                    let mut fields = Vec::new();
                    for f in header.split(',') {
                        fields.push(Field::new(f, DataType::UTF8));
                    }
                    Schema::new(fields, None)
                }
                Err(e) => return Err(Error::file_err(e, path)),
            }
        } else {
            Schema::new_single("line", DataType::UTF8)
        };
        let mut f = File {
            f,
            schema,
            path,
            read,
            write,
            create,
        };
        f.seek_start(0)?;
        Ok(f)
    }

    pub fn read_to_string(&mut self) -> Result<String> {
        let mut x = String::new();
        Self::map_err(self.f.read_to_string(&mut x), &self.path)?;
        Ok(x)
    }

    pub fn write_string(&mut self, content: &str) -> Result<()> {
        Self::map_err(self.f.write_all(content.as_bytes()), &self.path)?;
        Ok(())
    }

    pub fn seek_start(&mut self, pos: u64) -> Result<()> {
        Self::map_err(self.f.seek(SeekFrom::Start(pos)), &self.path)?;
        Ok(())
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = Tuple> + '_ {
        let buffer = BufReader::new(&self.f);

        buffer
            .lines()
            .skip(1)
            .scan((), |_, x| x.ok())
            .map(|x| x.split(',').map(Scalar::from).collect())
    }
}

pub fn read_file_to_string(f: &mut fs::File) -> Result<String> {
    let mut x = String::new();
    f.read_to_string(&mut x)?;
    Ok(x)
}

impl Rel for File {
    fn type_name(&self) -> &str {
        "File"
    }

    fn kind(&self) -> DataType {
        DataType::Vec(self.schema.kind().into())
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        if let Ok(m) = self.f.metadata() {
            m.len() as usize
        } else {
            0
        }
    }

    fn cols(&self) -> usize {
        self.schema.len()
    }

    fn rows(&self) -> Option<usize> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        RelShape::Vec
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher)
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        //TODO: Fix this ugly hack
        Self::new(self.path.clone(), self.read, self.write, self.create).unwrap()
    }
}

fn open(of: &[Scalar]) -> Result<Scalar> {
    if let Scalar::UTF8(name) = &of[0] {
        let f = File::new(name.as_str().into(), true, false, false)?;
        Ok(Scalar::File(f))
    } else {
        Err(Error::ParamTypeMismatch("open".into()))
    }
}

fn read_to_string(of: &[Scalar]) -> Result<Scalar> {
    if let Scalar::File(mut f) = of[0].clone() {
        let s = f.read_to_string()?;
        return Ok(s.into());
    };
    Err(Error::ParamTypeMismatch("read_to_string".into()))
}

fn save_file(of: &[Scalar]) -> Result<Scalar> {
    if of.len() != 2 {
        return Err(Error::ParamCount(of.len(), 2));
    }

    if let Scalar::File(mut f) = of[0].clone() {
        let s = format!("{}", &of[1]);
        f.write_string(&s)?;

        return Ok(Scalar::None);
    };
    Err(Error::ParamTypeMismatch("save_file".into()))
}

fn fn_open(name: &str, params: &[Param], f: RelFun) -> Function {
    Function::new(name, params, &[Param::kind(DataType::None)], Box::new(f))
}

pub fn functions() -> Vec<Function> {
    vec![
        fn_open("open", &[Param::kind(DataType::UTF8)], open),
        fn_open(
            "read_to_string",
            &[Param::kind(DataType::UTF8)],
            read_to_string,
        ),
        fn_open(
            "save",
            &[Param::kind(DataType::ANY), Param::kind(DataType::UTF8)],
            save_file,
        ),
    ]
}
