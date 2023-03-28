use std::collections::HashMap;
use crate::runtime::list::class::Class;
use crate::runtime::list::methods::{Method, MethodBody};
use crate::runtime::list::methods::inbuilt::*;
use crate::runtime::Value;
use crate::types::Type;

pub struct SymTab {
  table: HashMap<String, Symbol>
}

impl SymTab {
  pub fn new() -> Self {
    let mut new = Self {
      table: HashMap::new()
    };

    new.insert("int".to_string(), Symbol::Type(Type::Int));
    new.insert("bool".to_string(), Symbol::Type(Type::Bool));
    new.insert("char".to_string(), Symbol::Type(Type::Char));
    new.insert("float".to_string(), Symbol::Type(Type::Float));

    new.insert("add#int->int".to_string(), Symbol::Function(Box::new(Method {
      name: "add".to_string(),
      signature: vec!(Type::Int, Type::Int, Type::Int),
      body: MethodBody::Inbuilt(add_int_int_int),
    })));
    new
  }

  pub fn insert(&mut self, name : String, sym : Symbol) -> bool {
    if self.table.contains_key(&name) {
      return false;
    }
    self.table.insert(name, sym);
    true
  }

  pub fn get(&self, name : String) -> Option<&Symbol> {
    self.table.get(&name)
  }

  pub fn get_type(&self, name : String) -> Option<&Type> {
    match self.get(name) {
      Some(Symbol::Type(t)) => Some(t),
      _ => None
    }
  }

  pub fn get_class(&self, name : String) -> Option<&Class> {
    match self.get(name) {
      Some(Symbol::Type(Type::Class(c))) => Some(c),
      _ => None
    }
  }

  pub fn get_function(&self, name : String) -> Option<&Method> {
    match self.get(name) {
      Some(Symbol::Function(m)) => Some(m),
      _ => None
    }
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