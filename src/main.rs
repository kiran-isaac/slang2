mod runtime;
mod parser;
mod types;

use types::Type;
use runtime::{Class, List, ClassTable, Pattern, ListPtr};
use crate::runtime::list::list::Value;

fn add_builtins(table : &mut ClassTable) {
  let int_list = Class {name : "[int]".to_string(), supers : vec![], pattern : Pattern {
    types: vec![Type::Int],
    only: false
  }};
  let float_list = Class {name : "[float]".to_string(), supers : vec![], pattern : Pattern {
    types: vec![Type::Float],
    only: false
  }};
  let bool_list = Class {name : "[bool]".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Bool],
      only: false
  }};
  let char_list = Class {name : "[char]".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Char],
      only: false
  }};

  table.add_class("".to_string(), int_list);
  table.add_class("".to_string(), float_list);
  table.add_class("".to_string(), bool_list);
  table.add_class("".to_string(), char_list);
}

fn main() {
  let mut table = ClassTable::new();
  add_builtins(&mut table);

  let two_d_list = Class {
    name: "2DList".to_string(),
    supers: vec!["".to_string()],
    pattern: Pattern {
      types: vec![Type::Class(table.get_class("".to_string(), "[int]".to_string()).unwrap())],
      only: false
    }
  };
  table.add_class("".to_string(), two_d_list);

  // A list has a reference to a class. A list is equivalent to an object, so calling List::new
  // is basically just instantiation
  let mut list1 = List::new(table.get_class("".to_string(), "[int]".to_string()).unwrap());
  let mut list2 = List::new(table.get_class("".to_string(), "2DList".to_string()).unwrap());
  let mut list3 = List::new(table.get_class("".to_string(), "[int]".to_string()).unwrap());

  list1.push(Value::Int(10));
  list1.push(Value::Int(20));

  list3.take(&mut list1, 0, 1);

  list2.push(Value::List(ListPtr::new(list3)));
}
