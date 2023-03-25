mod runtime;
mod parser;
mod types;

use types::Type;
use runtime::{Class, List, ClassTable, Pattern};
use crate::runtime::list::list::PrimitiveValue;

fn main() {
  let mut test_class = Class {name : "Box(int)".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Int],
      only: true
  }};

  let mut table = ClassTable::new();
  table.add_class("".to_string(), &mut test_class);

  // A list has a reference to a class. A list is equivalent to an object, so calling List::new
  // is basically just instantiation
  let mut test_list = List::new(table.get_class("".to_string(), "Box(int)".to_string()).unwrap());

  test_list.push_primitive(PrimitiveValue::Int(10));
  test_list.push_primitive(PrimitiveValue::Int(20));
}
