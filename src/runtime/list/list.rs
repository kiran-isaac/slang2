use std::ptr::NonNull;
use crate::runtime::error::Error;
pub use super::{Class, ClassPtr};
pub use crate::runtime::list::value::{Value};

/// A list containing values
pub struct List {
  pub class : ClassPtr,
  val : Vec<Value>,
  accepting: usize
}

#[derive(Clone, Copy)]
pub struct ListPtr (NonNull<List>);

impl ListPtr {
  pub fn new(list : List) -> Self {
    Self(NonNull::new(Box::into_raw(Box::new(list))).unwrap())
  }

  pub fn get(&self) -> &List {
    unsafe { self.0.as_ref() }
  }
}

impl List {
  pub fn new(class : ClassPtr) -> List {
    List {
      val: Vec::new(),
      class,
      accepting: 0
    }
  }

  pub fn push(&mut self, val : Value) {
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
      println!(" - ERROR:");
      Error::TypeError(format!("Trying to push primitive value {} to object with pattern {} values", val.to_string(), self.class.get().pattern.to_string())).throw();
    }

    self.accepting += 1;
    self.val.push(val);
  }

  fn remove(&mut self, start : usize, end : usize) {
    self.val.drain(start..end);

    let start = start.clone() % self.class.get().pattern.types.len();

    for i in start..(start + self.class.get().pattern.types.len()) {
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

  // fn peek(&self, start : usize, end : usize) -> List {
  //   let mut list = List::new(self.class.clone());
  //   for i in start..end {
  //     list.push_primitive(self.val[i].clone());
  //   }
  //   list
  // }

  fn take_single(&mut self, other : &mut List, index : usize)  {
    let val: Value = other.val[index].clone();
    self.push(val);
    other.remove(index, index + 1);
  }

  pub fn take(&mut self, other : &mut List, start : usize, end : usize) {
    let mut values : Vec<Value> = Vec::new();
    for i in start..end {
      values.push(other.val[i]);
    }
    other.remove(start, end);
  }
}