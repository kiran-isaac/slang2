pub mod value;
pub mod pattern;

use crate::runtime::error::*;
use std::ptr::NonNull;

pub use pattern::Pattern;
pub use value::Value;

/// A list containing values
pub struct List {
  val : Vec<Value>, 
  class : ClassPtr,
  accepting: i32
}

impl List {
  fn take(&mut self, into : List, index : usize) {
    throw(Error::TypeError("nope".to_string()));
    
    // let pattern = self.structure.types;
    // let giving = pattern[index % pattern.len()];
  }
}