use crate::types::Type;

#[derive(Clone)]
pub struct Signature {
  types: Vec<Type>,
}

impl Signature {
  pub fn new(args: Vec<Type>) -> Self {
    Self { types: args }
  }

  pub fn get_params(&self) -> Vec<Type> {
    let mut copy = self.types.clone();
    copy.remove(copy.len() - 1);
    copy
  }

  pub fn get_return(&self) -> &Type {
    &self.types[self.types.len() - 1]
  }
}

impl PartialEq for Signature {
  fn eq(&self, other: &Self) -> bool {
    if self.types.len() != other.types.len() {
      return false;
    }
    for i in 0..self.types.len() {
      if self.types[i] != other.types[i] {
        return false;
      }
    }
    true
  }
}

impl ToString for Signature {
  fn to_string(&self) -> String {
    let mut s = String::new();
    for i in 0..self.types.len() - 2 {
      s.push_str(&self.types[i].to_string());
      s += " -> ";
    }
    s.push_str(&self.types[self.types.len() - 1].to_string());
    s
  }
}