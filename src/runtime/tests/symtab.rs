use crate::runtime::{Class};
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