use std::collections::HashMap;
use crate::runtime::list::class::{ClassPtr, Class};
use crate::runtime::Value;
use crate::types::Type;

pub struct SymbolTable {
  pub(crate) table: HashMap<String, Symbol>
}

#[derive(Clone)]
pub struct Symbol {
  pub type_: Type,
  pub value: Value,
}

#[allow(dead_code)]
impl SymbolTable {
  pub fn new() -> Self {
    Self {
      table: HashMap::new()
    }
  }

  pub fn init_with_defaults(&mut self) {
    self.table.insert("int".to_string(), Symbol {
      type_: Type::Class(ClassPtr::new(Class::anon_from_types(vec!(Type::Int)))),
      value: Value::Int(0)
    });
  }

  pub fn get_class(&self, name : String) -> Option<Symbol> {
    self.table.get(&name).cloned()
  }
}