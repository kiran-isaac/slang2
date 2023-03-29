use crate::runtime::list::List;

/// A value that can be stored on a list.
#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Value {
  List(Box<List>),
  Int(i64),
  Float(f64),
  Bool(bool),
  Char(char)
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::List(l) => l.to_string(),
      Value::Int(i) => format!("{}", i),
      Value::Float(f) => format!("{}", f),
      Value::Bool(b) => format!("{}", b),
      Value::Char(c) => format!("{}", c)
    }
  }
}