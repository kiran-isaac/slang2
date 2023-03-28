use crate::types::Type;

#[derive(Clone, PartialEq)]
pub struct Pattern {
  pub types: Vec<Type>,
  pub pattern_type: PatternType
}

#[derive(Clone, PartialEq)]
pub enum PatternType{
  Only,
  None,
  Repeating
}

impl ToString for Pattern {
  fn to_string(&self) -> String {
    if self.pattern_type == PatternType::None { return "_".to_string(); }
    let mut s = String::new();
    let mut empty = true;
    for t in &self.types {
      s += &t.to_string();
      s += ", ";
      empty = false;
    }
    if !empty { s.drain(s.len() - 2..); }
    if self.pattern_type == PatternType::Only { s } else { s + "..." }
  }
}