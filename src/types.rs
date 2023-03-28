use crate::runtime::{Class, list::Signature, Value};

/// A type that a value may have. Either an object with a class name, or a primitive type.
#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Type {
  Union(Box<Type>, Box<Type>),
  Class(Box<Class>),
  Method(Signature),
  Char,
  Int, 
  Float, 
  Bool
}

impl ToString for Type {
  fn to_string(&self) -> String {
    match self {
      Type::Union(t1, t2) => format!("{} | {}", t1.to_string(), t2.to_string()),
      Type::Class(s) => s.to_string(),
      Type::Method(s) => s.to_string(),
      Type::Char => "char".to_string(),
      Type::Int => "int".to_string(),
      Type::Float => "float".to_string(),
      Type::Bool => "bool".to_string()
    }
  }
}

impl Type {
  pub fn is_of_type(&self, t : &Value) -> bool {
    match self {
      Type::Union(t1, t2) => t1.is_of_type(t) || t2.is_of_type(t),
      Type::Class(class) => if let Value::List(list) = t {
        *class == list.class
      } else {false},
      Type::Char => if let Value::Char(_) = t {
        true
      } else {false},
      Type::Int => if let Value::Int(_) = t {
        true
      } else {false},
      Type::Float => if let Value::Float(_) = t {
        true
      } else {false},
      Type::Bool => if let Value::Bool(_) = t {
        true
      } else {false}
      _ => false
    }
  }
}