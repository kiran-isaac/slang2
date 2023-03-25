use crate::types::Type;

pub struct Pattern {
  pub types: Vec<Type>,
  pub only: bool
}

/// A class that a list may be instantiated from
pub struct Class {
  pub name: String,
  pub supers: Vec<String>,
  pub pattern: Pattern
}

impl ToString for Pattern {
  fn to_string(&self) -> String {
    let mut s = String::new();
    if self.only {
      s += "ONLY(";
    }
    for t in &self.types {
      s += &t.to_string();
    }
    s + ")"
  }
}