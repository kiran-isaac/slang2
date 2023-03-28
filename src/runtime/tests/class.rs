use crate::runtime::{Class, Pattern, PatternType};
use crate::types::Type;

#[test]
pub fn class_to_string() {
  let p1 = Pattern {
    types: vec!(),
    pattern_type: PatternType::Repeating
  };

  let p2 = Pattern {
    types: vec!(Type::Int, Type::Bool),
    pattern_type: PatternType::Repeating
  };

  let c1 = Box::new(Class::anon_from_pattern(p1));
  let c2 = Box::new(Class::anon_from_pattern(p2));

  let container = Class::anon_from_pattern(Pattern {
    types: vec!(Type::Class(c1.clone()), Type::Class(c2.clone())),
    pattern_type: PatternType::Repeating
  });

  assert_eq!(c1.to_string(), "{_ of (...)}");
  assert_eq!(c2.to_string(), "{_ of (int, bool...)}");

  println!("{}", container.to_string());
}