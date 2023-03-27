use super::Method;
use std::ptr::{eq, NonNull};

#[derive(Clone, Copy, Debug)]
pub struct MethodPtr(NonNull<Method>);

#[allow(dead_code)]
impl MethodPtr {
  pub fn new(class : Method) -> Self {
    Self(NonNull::new(Box::into_raw(Box::new(class))).unwrap())
  }

  pub fn get(&self) -> &Method {
    unsafe { self.0.as_ref() }
  }
}

impl PartialEq for MethodPtr {
  fn eq(&self, other: &Self) -> bool {
    eq(self.0.as_ptr(), other.0.as_ptr())
  }
}