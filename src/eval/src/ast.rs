pub type SpanId = u64;

pub type Compiled = Box<dyn Fn(&mut Vec<Expr>) -> Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Pass(SpanId),
    Eof(SpanId),
}
