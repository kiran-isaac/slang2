use crate::runtime::error::Error;
pub use crate::runtime::list::class::{Class};
pub use crate::runtime::list::value::{PrimitiveValue};
use crate::types::Type;

/// A list containing values
pub struct List<'a> {
  val : Vec<PrimitiveValue>,
  class : &'a Class,
  accepting: usize
}

impl List<'_> {
  pub fn new(class : &Class) -> List {
    List {
      val: Vec::new(),
      class,
      accepting: 0
    }
  }

  pub fn push_primitive(&mut self, val : PrimitiveValue) {
    if self.accepting == self.class.pattern.types.len() {
      if self.class.pattern.only {
        Error::TypeError(format!("Trying to append value {} to object with pattern {} values", val.to_string(), self.class.pattern.to_string())).throw();
      } else {
        self.accepting = 0;
      }
    }

    let expect = &self.class.pattern.types[self.accepting.clone()];
    println!("Expecting {}, got {}, {}", expect.to_string(), val.to_string(), val.is_of_type(expect));

    self.accepting += 1;
    self.val.push(val);
  }
}