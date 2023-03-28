use crate::runtime::{Class, Value};
use crate::runtime::symtab::*;
use crate::types::Type;

#[test]
pub fn initial_symtab_test() {
  let mut symtab = SymTab::new();

  assert!(symtab.get("a".to_string()).is_none());

  symtab.insert("a".to_string(), Symbol::from_class(Class::anon_from_types(vec!(Type::Int))));

  assert!(symtab.get("a".to_string()).is_some());

  let result = symtab.get("a".to_string()).unwrap();

  if let Symbol::Type(Type::Class(class)) = result {
    println!("Class: {}", class.to_string())
  } else {
    panic!("Expected class")
  }
}

#[test]
pub fn builtin_types_test() {
  let mut symtab = SymTab::new();

  let int_type = symtab.get("int".to_string());
  if int_type.is_none() {
    panic!("Expected int type");
  }

  assert!(symtab.insert("x".to_string(), Symbol::Value(Value::Int(5))))
}