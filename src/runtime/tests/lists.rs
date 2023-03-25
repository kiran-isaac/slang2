use crate::runtime::*;
use crate::runtime::list::list::Value;
use crate::types::Type;

fn init_testing_class_table() -> ClassTable {
  let mut class_table = ClassTable::new();

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

  let is_even = Class {name : "IsEven".to_string(), supers : vec![], pattern : Pattern {
      types: vec![Type::Int, Type::Bool],
      only: false
  }};

  class_table.add_class("".to_string(), int_list);
  class_table.add_class("".to_string(), float_list);
  class_table.add_class("".to_string(), bool_list);
  class_table.add_class("".to_string(), char_list);

  class_table.add_class("".to_string(), is_even);

  class_table
}

#[test]
pub fn will_fail() {
  let test_table = init_testing_class_table();

  let mut is_even = List::new(test_table.get_class("".to_string(), "IsEven".to_string()).unwrap());
  is_even.push(Value::Int(0));
  is_even.push(Value::Bool(true));
  is_even.push(Value::Int(1));
  is_even.push(Value::Bool(false));

  let mut int_list = List::new(test_table.get_class("".to_string(), "[int]".to_string()).unwrap());
  int_list.take(&mut is_even, 0, 1);
}