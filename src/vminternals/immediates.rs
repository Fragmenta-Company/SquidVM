
#[derive(Clone, Debug)]
pub struct BinaryCode(i128);

#[derive(Clone)]
pub enum Immediates {
    Null,
    Boolean(bool),
    UInteger(u64),
    Integer(i64),
    Float(f64),
    String(String),
    Binary(BinaryCode),
    Enum(Box<[Immediates]>),
}
