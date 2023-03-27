use std::collections::HashMap;
use crate::runtime::list::class::Class;
use crate::runtime::list::methods::Method;
use crate::runtime::Value;
use crate::types::Type;

pub struct SymTab {
  table: HashMap<String, Symbol>
}

impl SymTab {
  pub fn new() -> Self {
    Self {
      table: HashMap::new()
    }
  }

  pub fn insert(&mut self, name : String, sym : Symbol) {
    self.table.insert(name, sym);
  }

  pub fn get(&self, name : String) -> Option<&Symbol> {
    self.table.get(&name)
  }
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Symbol {
  Type(Type),
  Function(Box<Method>),
  Value(Value),
}

impl Symbol {
  pub fn from_class(class : Class) -> Self {
    Symbol::Type(Type::Class(Box::new(class)))
  }
}