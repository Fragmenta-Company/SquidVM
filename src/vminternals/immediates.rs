
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Immediates {
    Null,
    Boolean(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    String(String),
}
