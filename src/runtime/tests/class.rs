use crate::runtime::{Class, Pattern, ClassPtr};
use crate::types::Type;

#[test]
pub fn class_to_string() {
  let p1 = Pattern {
    types: vec!(),
    only: false
  };

  let p2 = Pattern {
    types: vec!(Type::Int, Type::Bool),
    only: false
  };

  let c1 = ClassPtr::new(Class::anon_from_pattern(p1));
  let c2 = ClassPtr::new(Class::anon_from_pattern(p2));

  let container = Class::anon_from_pattern(Pattern {
    types: vec!(Type::Class(c1), Type::Class(c2)),
    only: false
  });

  assert_eq!(c1.get().to_string(), "{_ of (...)}");
  assert_eq!(c2.get().to_string(), "{_ of (int, bool...)}");

  println!("{}", container.to_string());
}