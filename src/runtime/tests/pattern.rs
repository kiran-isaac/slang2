use crate::runtime::Pattern;
use crate::types::Type;

#[test]
pub fn pattern_to_string() {
  let p1 = Pattern {
    types: vec!(Type::Int, Type::Bool),
    only: true
  };

  let p2 = Pattern {
      types: vec!(Type::Int, Type::Bool),
      only: false
  };

  assert_eq!(p1.to_string(), "int, bool");
  assert_eq!(p2.to_string(), "int, bool...");
}