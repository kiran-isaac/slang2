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
        Error::TypeError(format!("Trying to push primitive value {} to FULL object with pattern {} values", val.to_string(), self.class.pattern.to_string())).throw();
      } else {
        self.accepting = 0;
      }
    }

    let expect = &self.class.pattern.types[self.accepting.clone()];
    print!("Expecting {}, got {}", expect.to_string(), val.to_string());
    if val.is_of_type(expect) {
      println!(" - OK");
    } else {
      println!(" - ERROR");
      Error::TypeError(format!("Trying to push primitive value {} to object with pattern {} values", val.to_string(), self.class.pattern.to_string())).throw();
    }

    self.accepting += 1;
    self.val.push(val);
  }
}