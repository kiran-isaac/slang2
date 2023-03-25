use crate::runtime::error::Error;
pub use crate::runtime::list::class::{Class, ClassPtr};
pub use crate::runtime::list::value::{PrimitiveValue};

/// A list containing values
pub struct List {
  val : Vec<PrimitiveValue>,
  class : ClassPtr,
  accepting: usize
}

impl List {
  pub fn new(class : ClassPtr) -> List {
    List {
      val: Vec::new(),
      class,
      accepting: 0
    }
  }

  pub fn push_primitive(&mut self, val : PrimitiveValue) {
    if self.accepting == self.class.get().pattern.types.len() {
      if self.class.get().pattern.only {
        Error::TypeError(format!("Trying to push primitive value {} to FULL object with pattern {} values", val.to_string(), self.class.get().pattern.to_string())).throw();
      } else {
        self.accepting = 0;
      }
    }

    let expect = &self.class.get().pattern.types[self.accepting.clone()];
    print!("Expecting {}, got {}", expect.to_string(), val.to_string());
    if val.is_of_type(expect) {
      println!(" - OK");
    } else {
      println!(" - ERROR");
      Error::TypeError(format!("Trying to push primitive value {} to object with pattern {} values", val.to_string(), self.class.get().pattern.to_string())).throw();
    }

    self.accepting += 1;
    self.val.push(val);
  }
}