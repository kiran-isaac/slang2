use crate::runtime::{List, Value};
use crate::types::Type;

#[test]
pub fn test() {
  let mut l1 = List::new_repeating_of(vec!(Type::Union(Box::new(Type::Int), Box::new(Type::Bool))));

  l1.push(Value::Int(5));
  l1.push(Value::Bool(true));
}