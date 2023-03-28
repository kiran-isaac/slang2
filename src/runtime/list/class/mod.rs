mod pattern;

use crate::types::Type;
pub use pattern::{Pattern, PatternType};

/// A class that a list may be instantiated from
#[derive(Clone, PartialEq)]
pub struct Class {
  pub name: String,
  pub supers: Vec<String>,
  pub pattern: Pattern
}

impl Class {
  pub fn anon_from_types(of : Vec<Type>) -> Self {
    let pattern = Pattern {
      types: of,
      pattern_type: PatternType::Repeating
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