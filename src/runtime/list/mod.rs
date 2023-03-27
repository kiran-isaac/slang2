pub mod value;
pub mod class;
pub mod list_pointer;
pub mod methods;

pub use value::Value;
pub use class::*;
pub use list_pointer::{ListPtr};
pub use methods::Signature;

use crate::runtime::error::Error;
use crate::types::Type;

/// A list containing values
pub struct List {
  pub class: ClassPtr,
  val: Vec<Value>,
  accepting: usize,
}

/// This implementation will be used for debugging purposes
/// It will print the list in the format [val1, val2, val3, ...]
/// If the value is a list, it will print it in the same format
impl ToString for List {
  fn to_string(&self) -> String {
    let mut s = String::new();
    s += "[";
    let mut empty = true;
    for v in &self.val {
      s += &v.to_string();
      s += ", ";
      empty = false;
    }
    if !empty { s.drain(s.len() - 2..); };
    s + "]"
  }
}

#[allow(dead_code)]
impl List {
  pub fn new(class: ClassPtr) -> List {
    List {
      val: Vec::new(),
      class,
      accepting: 0,
    }
  }

  /// Creates a list with a repeating pattern of the given types
  pub fn new_repeating_of(pattern: Vec<Type>) -> List {
    List::new(ClassPtr::new(Class::anon_from_types(pattern)))
  }

  /// Creates a list a list with a single pattern of the given types
  pub fn new_single_of(pattern: Vec<Type>) -> List {
    List::new(ClassPtr::new(Class::anon_from_pattern(Pattern {types : pattern, only : true})))
  }

  /// Creates a list with with a repeating pattern of any type
  pub fn new_repeating_of_any() -> List {
    List::new_repeating_of(vec!())
  }

  /// Creates a list with a single pattern of any type
  pub fn new_single_of_any() -> List {
    List::new_single_of(vec!())
  }

  pub fn insert(&mut self, val: Value, index: usize) {
    if self.class.get().pattern.types.len() != 0 {

    }
    self.val.insert(index, val);
  }

  pub fn push(&mut self, val: Value) {
    // If the list has a pattern, verify that the value matches the pattern
    if self.class.get().pattern.types.len() != 0 {
      // If the list is full, verify that it is not an only pattern
      // Otherwise, reset the accepting index
      if self.accepting == self.class.get().pattern.types.len() {
        if self.class.get().pattern.only {
          Error::TypeError(format!("Trying to push primitive value {} to FULL object with pattern {} values", val.to_string(), self.class.get().pattern.to_string())).throw();
        } else {
          self.accepting = 0;
        }
      }

      // Verify that the pattern matches. This only needs to be checked for the value pushed
      let expect = &self.class.get().pattern.types[self.accepting.clone()];
      if !val.is_of_type(expect) {
        Error::TypeError(format!("Trying to push value {} to object with pattern {} values", val.to_string(), self.class.get().pattern.to_string())).throw();
      }

      self.accepting += 1;
    }
    self.val.push(val);
  }

  fn remove(&mut self, start: usize, end: usize) {
    self.val.drain(start..end);

    // Verify pattern integrity is maintained after removal
    for i in start..(start + self.class.get().pattern.types.len()) {
      if i >= self.val.len() { break; };
      let expect = &self.class.get().pattern.types[i % self.class.get().pattern.types.len()];
      let got = &self.val[i];
      if !got.is_of_type(expect) {
        if start == end {
          Error::TypeError(format!("Removing element {} from a '{}' with pattern {} causes a pattern mismatch. Try peeking, or removing a whole segment"
                                   , start, self.class.get().name, self.class.get().pattern.to_string())).throw();
        } else {
          Error::TypeError(format!("Removing elements [{}, {}] from a '{}' with pattern {} causes a pattern mismatch. Try peeking, or removing a whole segment"
                                   , start, end, self.class.get().name, self.class.get().pattern.to_string())).throw();
        }
      }
    }
  }

  pub fn take_end_segment(&mut self, from: &mut List) {
    let pattern_len = from.class.get().pattern.types.len();
    let start = from.val.len() - pattern_len;
    let end = from.val.len();
    self.take(from, start, end);
  }

  pub fn take(&mut self, from: &mut List, start: usize, end: usize) {
    for i in start..end {
      self.val.push(from.val[i].clone());
    }
    from.remove(start, end);
  }
}