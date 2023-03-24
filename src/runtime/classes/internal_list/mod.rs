use crate::runtime::error::*;
use std::ptr::NonNull;

pub mod value;

/// A list containing values and a reference to its class.
pub struct List {
  val : Vec<Value>, 
  structure: Pattern,
  accepting: i32
}

/// A pointer to a list. These are managed by the runtime.
pub struct ListPtr(NonNull<List>);

impl List {
  fn take(&mut self, into : List, index : usize) {
    throw(Error::TypeError("nope".to_string()));
    
    let pattern = self.structure.types;
    let giving = pattern[index % pattern.len()];
  }
}