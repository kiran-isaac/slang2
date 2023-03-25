use crate::runtime::list::list::ListPtr;
use crate::types::Type;

/// A value that can be stored on a list.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Value {
  List(ListPtr),
  Int(i64),
  Float(f64),
  Bool(bool)
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Value::List(l) => l.get().class.get().to_string(),
      Value::Int(i) => format!("int({})", i),
      Value::Float(f) => format!("float({})", f),
      Value::Bool(b) => format!("bool({})", b)
    }
  }
}

impl Value {
  pub fn is_of_type(&self, t : &Type) -> bool {
    match self {
      Value::List(list) => if let Type::Class(class) = t {
        *class == list.get().class
      } else {false},
      Value::Int(_) => *t == Type::Int,
      Value::Float(_) => *t == Type::Float,
      Value::Bool(_) => *t == Type::Bool
    }
  }
}