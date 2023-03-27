mod pattern;
mod class_pointer;

use crate::types::Type;
pub use pattern::Pattern;
pub use class_pointer::ClassPtr;

/// A class that a list may be instantiated from
#[derive(Clone)]
pub struct Class {
  pub name: String,
  pub supers: Vec<String>,
  pub pattern: Pattern
}

impl Class {
  pub fn anon_from_types(of : Vec<Type>) -> Self {
    let pattern = Pattern {
      types: of,
      only: false
    };
    Class {
      name: "".to_string(),
      supers: vec![],
      pattern
    }
  }

  pub fn anon_from_pattern(pattern : Pattern) -> Self {
    Class {
      name: "".to_string(),
      supers: vec![],
      pattern
    }
  }
}

impl ToString for Class {
  fn to_string(&self) -> String {
    let name = if self.name == "" { "_".to_string() } else { self.name.clone() };
    "{".to_string() + &name + " of (" + &self.pattern.to_string() + ")}"
  }
}