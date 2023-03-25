use crate::types::Type;

/// A value that can be stored on a list.
pub enum PrimitiveValue {
  Int(i64),
  Float(f64),
  Bool(bool)
}

impl ToString for PrimitiveValue {
  fn to_string(&self) -> String {
    match self {
      PrimitiveValue::Int(i) => format!("int({})", i),
      PrimitiveValue::Float(f) => format!("float({})", f),
      PrimitiveValue::Bool(b) => format!("bool({})", b)
    }
  }
}

impl PrimitiveValue {
  pub fn is_of_type(&self, t : &Type) -> bool {
    match self {
      PrimitiveValue::Int(_) => *t == Type::Int,
      PrimitiveValue::Float(_) => *t == Type::Float,
      PrimitiveValue::Bool(_) => *t == Type::Bool
    }
  }
}