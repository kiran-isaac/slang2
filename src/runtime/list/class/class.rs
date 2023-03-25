use crate::types::Type;
use std::ptr::{NonNull, eq};

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

#[derive(Clone, Copy)]
pub struct ClassPtr (NonNull<Class>);

impl ClassPtr {
  pub fn new(class : Class) -> Self {
    Self(NonNull::new(Box::into_raw(Box::new(class))).unwrap())
  }


  pub fn get(&self) -> &Class {
    unsafe { self.0.as_ref() }
  }
}

impl PartialEq for ClassPtr {
  fn eq(&self, other: &Self) -> bool {
    eq(self.0.as_ptr(), other.0.as_ptr())
  }
}

impl ToString for Pattern {
  fn to_string(&self) -> String {
    let mut s = String::new();
    if self.only {
      s += "ONLY ";
    }
    for t in &self.types {
      s += &t.to_string();
    }
    s
  }
}

impl ToString for Class {
  fn to_string(&self) -> String {
    self.name.to_string() + "(" + &self.pattern.to_string() + ")"
  }
}