use crate::runtime::{Pattern, PatternType};
use crate::types::Type;

#[test]
pub fn pattern_to_string() {
  let p1 = Pattern {
    types: vec!(Type::Int, Type::Bool),
    pattern_type: PatternType::Only,
  };

  let p2 = Pattern {
    types: vec!(Type::Int, Type::Bool),
    pattern_type: PatternType::Repeating,
  };

  assert_eq!(p1.to_string(), "int, bool");
  assert_eq!(p2.to_string(), "int, bool...");
}

#[test]
pub fn union_test() {
  let u1 = Type::Union(Box::new(Type::Int), Box::new(Type::Bool));


}