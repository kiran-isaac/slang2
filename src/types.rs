/// A type that a value may have. Either an object with a class name, or a primitive type.
#[derive(Clone, PartialEq)]
pub enum Type {
  List(String),
  Char,
  Int, 
  Float, 
  Bool
}

impl ToString for Type {
  fn to_string(&self) -> String {
    match self {
      Type::List(s) => s.to_string(),
      Type::Char => "char".to_string(),
      Type::Int => "int".to_string(),
      Type::Float => "float".to_string(),
      Type::Bool => "bool".to_string()
    }
  }
}