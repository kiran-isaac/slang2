mod runtime;
mod parser;
mod types;

use types::Type;
use runtime::{Class, List, ClassTable, Pattern};
use crate::runtime::list::list::PrimitiveValue;

fn add_builtins(table : &mut ClassTable) {
  let int_list = Class {name : "[int]".to_string(), supers : vec![], pattern : Pattern {
    types: vec![Type::Int],
    only: true
  }};
  let float_list = Class {name : "[float]".to_string(), supers : vec![], pattern : Pattern {
    types: vec![Type::Float],
    only: true
  }};
  let bool_list = Class {name : "[bool]".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Bool],
      only: true
  }};
  let char_list = Class {name : "[char]".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Char],
      only: true
  }};

  table.add_class("".to_string(), int_list);
  table.add_class("".to_string(), float_list);
  table.add_class("".to_string(), bool_list);
  table.add_class("".to_string(), char_list);
}

fn main() {
  let box_of_int = Class {name : "Box(int)".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Int],
      only: true
  }};

  let mut table = ClassTable::new();
  add_builtins(&mut table);
  table.add_class("".to_string(), box_of_int);

  // A list has a reference to a class. A list is equivalent to an object, so calling List::new
  // is basically just instantiation
  let mut list1 = List::new(table.get_class("".to_string(), "[int]".to_string()).unwrap());
  let mut list2 = List::new(table.get_class("".to_string(), "[int]".to_string()).unwrap());

  list1.push_primitive(PrimitiveValue::Int(10));
  list2.push_primitive(PrimitiveValue::Int(20));
}
