pub mod value;
pub mod class;
pub mod methods;

pub use value::Value;
pub use class::*;
pub use methods::Signature;

use crate::runtime::error::Error;
use crate::types::Type;

/// A list containing values
#[derive(Clone, PartialEq)]
pub struct List {
  pub class: Box<Class>,
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
  pub fn new(class: Box<Class>) -> List {
    List {
      val: Vec::new(),
      class,
      accepting: 0,
    }
  }

  /// Creates a list with a repeating pattern of the given types
  pub fn new_repeating_of(pattern: Vec<Type>) -> List {
    List::new(Box::new(Class::anon_from_types(pattern)))
  }

  /// Creates a list a list with a single pattern of the given types
  pub fn new_single_of(pattern: Vec<Type>) -> List {
    List::new(Box::new(Class::anon_from_pattern(Pattern {types : pattern, pattern_type : PatternType::Only})))
  }

  /// Creates a list with with a repeating pattern of any type
  pub fn new_repeating_of_any() -> List {
    List::new_repeating_of(vec!())
  }

  /// Creates a list with a single pattern of any type
  pub fn new_single_of_any() -> List {
    List::new_single_of(vec!())
  }

  pub fn push(&mut self, val: Value) -> Result<(), Error> {
    // If the list has a pattern, verify that the value matches the pattern
    if self.class.pattern.types.len() != 0 {
      // If the list is full, verify that it is not an only pattern
      // Otherwise, reset the accepting index
      if self.accepting == self.class.pattern.types.len() {
        if self.class.pattern.pattern_type == PatternType::Only {
          return Err(Error::TypeError(format!("Trying to push primitive value {} to FULL object with pattern {} values", val.to_string(), self.class.pattern.to_string())));
        } else {
          self.accepting = 0;
        }
      }

      // Verify that the pattern matches. This only needs to be checked for the value pushed
      let expect = &self.class.pattern.types[self.accepting.clone()];
      if !expect.is_of_type(&val) {
        Some(Error::TypeError(format!("Trying to push value {} to object with pattern {} values", val.to_string(), self.class.pattern.to_string())));
      }

      self.accepting += 1;
    }
    self.val.push(val);
    Ok(())
  }

  fn remove(&mut self, start: usize, end: usize) -> Result<(), Error> {
    self.val.drain(start..end);

    // Verify pattern integrity is maintained after removal
    for i in start..(start + self.class.pattern.types.len()) {
      if i >= self.val.len() { break; };
      let expect = &self.class.pattern.types[i % self.class.pattern.types.len()];
      let got = &self.val[i];
      if expect.is_of_type(got) {
        if start == end {
          return Err(Error::TypeError(format!("Removing element {} from a '{}' with pattern {} causes a pattern mismatch. Try peeking, or removing a whole segment"
                                   , start, self.class.name, self.class.pattern.to_string())));
        } else {
          return Err(Error::TypeError(format!("Removing elements [{}, {}] from a '{}' with pattern {} causes a pattern mismatch. Try peeking, or removing a whole segment"
                                   , start, end, self.class.name, self.class.pattern.to_string())));
        }
      }
    }
    Ok(())
  }

  pub fn take_end_segment(&mut self, from: &mut List) -> Result<(), Error> {
    let pattern_len = from.class.pattern.types.len();
    let start = from.val.len() - pattern_len;
    let end = from.val.len();
    self.take(from, start, end)
  }

  pub fn take(&mut self, from: &mut List, start: usize, end: usize) -> Result<(), Error> {
    for i in start..end {
      self.push(from.val[i].clone())?;
    }
    from.remove(start, end)
  }

  pub fn index(&self, index: i32) -> Result<Value, Error> {
    if index >= self.val.len() as i32 {
      return Err(Error::SliceError(format!("Trying to index list at {} when the list is only {} elements long", index, self.val.len())));
    }

    let index = if index < 0 { self.val.len() as i32 + index } else { index };
    if index < 0 {
      return Err(Error::SliceError("Trying to index list with a negative index greater than the length of the list".to_string()));
    }
    Ok(self.val[index as usize].clone())
  }

  pub fn slice(&self, start: Option<i32>, end: Option<i32>) -> Result<List, Error> {
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(self.val.len() as i32);

    let start = if start < 0 { self.val.len() as i32 + start } else { start };
    let end = if end < 0 { self.val.len() as i32 + end } else { end };

    if (start > end) && (end >= self.val.len() as i32) {
      return Err(Error::SliceError(format!("Trying to slice list from {} to {} when the list is only {} elements long", start, end, self.val.len())));
    }

    if start < 0 {
      return Err(Error::SliceError("Trying to slice list with a starting from before 0".to_string()));
    }

    let mut new = List::new(self.class.clone());
    for i in start..end {
      new.val.push(self.val[i as usize].clone());
    }
    Ok(new)
  }
}