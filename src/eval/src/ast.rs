pub type Return = std::result::Result<Expr, ()>;

pub type SpanId = u64;

#[derive(Debug, Clone)]
pub enum Expr {
    Pass(SpanId),
    Eof(SpanId),
}
