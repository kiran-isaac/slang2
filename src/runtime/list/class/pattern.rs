use crate::types::Type;

pub struct Pattern {
  pub types: Vec<Type>,
  pub only: bool
}

impl ToString for Pattern {
  fn to_string(&self) -> String {
    let mut s = String::new();
    let mut empty = true;
    for t in &self.types {
      s += &t.to_string();
      s += ", ";
      empty = false;
    }
    if !empty { s.drain(s.len() - 2..); }
    if self.only { s } else { s + "..." }
  }
}