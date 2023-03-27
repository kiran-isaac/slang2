use crate::runtime::list::List;
use crate::types::Type;

/// A value that can be stored on a list.
#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Value {
  List(Box<List>),
  Int(i64),
  Float(f64),
  Bool(bool)
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::List(l) => l.to_string(),
      Value::Int(i) => format!("{}", i),
      Value::Float(f) => format!("{}", f),
      Value::Bool(b) => format!("{}", b)
    }
  }
}

impl Value {
  pub fn is_of_type(&self, t : &Type) -> bool {
    match self {
      Value::List(list) => if let Type::Class(class) = t {
        *class == list.class
      } else {false},
      Value::Int(_) => *t == Type::Int,
      Value::Float(_) => *t == Type::Float,
      Value::Bool(_) => *t == Type::Bool
    }
  }
}