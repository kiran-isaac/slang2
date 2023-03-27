use super::Class;
use std::ptr::{eq, NonNull};

#[derive(Clone, Copy)]
pub struct ClassPtr (NonNull<Class>);

impl ClassPtr {
  pub fn new(class : Class) -> Self {
    Self(NonNull::new(Box::into_raw(Box::new(class))).unwrap())
  }

  pub fn get(&self) -> &Class {
    unsafe { self.0.as_ref() }
  }
}

impl PartialEq for ClassPtr {
  fn eq(&self, other: &Self) -> bool {
    eq(self.0.as_ptr(), other.0.as_ptr())
  }
}