use crate::for_impl::*;
use crate::prelude::*;

use derive_more::{Display, From};

pub enum FunCall<'a> {
    Nullary,
    Unary(&'a Scalar),
    Binary(&'a Scalar, &'a Scalar),
    Many(&'a [Scalar]),
}

impl<'a> FunCall<'a> {
    fn len(&self) -> usize {
        match self {
            FunCall::Nullary => 0,
            FunCall::Unary(_) => 1,
            FunCall::Binary(_, _) => 2,
            FunCall::Many(x) => x.len(),
        }
    }

    fn unpack_binary(&self) -> (&Scalar, &Scalar) {
        if let FunCall::Binary(a, b) = self {
            (a, b)
        } else {
            unreachable!()
        }
    }

    fn kind(&self) -> Vec<DataType> {
        match self {
            FunCall::Nullary => vec![],
            FunCall::Unary(x) => vec![x.kind()],
            FunCall::Binary(a, b) => vec![a.kind(), b.kind()],
            FunCall::Many(x) => x.iter().map(|x| x.kind()).collect(),
        }
    }
}

pub trait RelFun: for<'a> Fn(FunCall<'a>) -> ResultT<Scalar> {
    fn clone_object(&self) -> Box<dyn RelFun>;
}

impl<F> RelFun for F
where
    F: 'static + Clone + for<'a> Fn(FunCall<'a>) -> ResultT<Scalar>,
{
    fn clone_object(&self) -> Box<dyn RelFun> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RelFun> {
    fn clone(&self) -> Self {
        self.clone_object()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Params<'a> {
    pub params: &'a [Scalar],
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Display)]
#[display(fmt = "Fun()")]
pub struct FunctionDec {
    pub name: String,
    pub fields: Vec<Field>,
    pub result: Field,
}

impl FunctionDec {
    pub fn new(name: &str, fields: &[Field], result: &Field) -> Self {
        FunctionDec {
            name: name.to_string(),
            fields: fields.to_vec(),
            result: result.clone(),
        }
    }

    pub fn new_bin_op(name: &str, left: &str, right: &str, kind: DataType) -> Self {
        let lhs = Field::new(left, kind.clone());
        let rhs = Field::new(right, kind.clone());
        let ret = Field::new("", kind);

        Self::new(name, &[lhs, rhs], &ret)
    }

    pub fn new_single(name: &str, field: Field, ret: DataType) -> Self {
        let ret = Field::new("", ret);

        Self::new(name, &[field], &ret)
    }

    pub fn new_variadic(name: &str, kind: DataType) -> Self {
        Self::new_single(
            name,
            Field::new_positional(DataType::Variadic(Box::new(kind))),
            DataType::Any,
        )
    }

    pub fn new_for_not_found(name: &str, params: FunCall) -> Self {
        let fields = params
            .kind()
            .iter()
            .map(|x| Field::new_positional(x.clone()))
            .collect::<Vec<_>>();

        Self::new(name, &fields, &Field::new_positional(DataType::Any))
    }

    pub fn check_call(&self, params: &FunCall) -> ResultT<()> {
        if params.len() != self.fields.len() {
            return Err(Error::ParamCount(params.len(), self.fields.len()));
        }

        Ok(())
    }
}

#[derive(Clone, From, Display)]
#[display(fmt = "Fun()")]
pub struct Function {
    pub head: FunctionDec,
    f: Box<dyn RelFun>,
}

impl Function {
    pub fn new(head: FunctionDec, f: Box<dyn RelFun>) -> Self {
        Function { head, f }
    }

    pub fn call(&self, params: FunCall) -> ResultT<Scalar> {
        (self.f)(params)
    }

    pub fn key(&self) -> String {
        let mut key = String::new();
        key += &self.head.name;
        for p in &self.head.fields {
            key += &*format!("_{}", p.kind);
        }
        key
    }
}

impl Rel for Function {
    fn type_name(&self) -> &str {
        "Fun"
    }

    fn kind(&self) -> DataType {
        DataType::Fun(self.into())
    }

    fn schema(&self) -> Schema {
        Schema::new(self.head.fields.clone(), None)
    }

    fn len(&self) -> usize {
        0
    }

    fn size(&self) -> ShapeLen {
        ShapeLen::Iter(1, None)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

    fn iter(&self) -> Box<IterScalar<'_>> {
        unimplemented!()
    }

    fn col(&self, pos: usize) -> Col<'_> {
        todo!()
    }

    fn rows(&self) -> Box<IterRows<'_>> {
        unimplemented!()
    }
}

pub trait IntoFunction<Args, Out> {
    fn into_fun(self) -> (usize, Box<dyn RelFun>);
}

impl<F, A, B, Out> IntoFunction<(A, B), Out> for F
where
    A: From<Scalar>,
    B: From<Scalar>,
    Out: Into<Scalar>,
    F: Fn(A, B) -> Out + Clone + 'static,
{
    fn into_fun(self) -> (usize, Box<dyn RelFun>) {
        let f = move |args: FunCall| {
            assert_eq!(args.len(), 2);
            let (a, b) = args.unpack_binary();
            let a: A = a.clone().into();
            let b: B = b.clone().into();
            Ok((self)(a, b).into())
        };
        (2, Box::new(f))
    }
}

impl fmt::Debug for FunctionDec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fun {}({:?})={:?}", self.name, self.fields, self.result)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.head)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }
}

impl Eq for Function {}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.head.cmp(&other.head))
    }
}

impl Ord for Function {
    fn cmp(&self, other: &Self) -> Ordering {
        self.head.cmp(&other.head)
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.head.hash(state);
    }
}
