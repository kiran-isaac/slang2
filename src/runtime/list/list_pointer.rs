use super::List;
use std::ptr::{eq, NonNull};

#[derive(Clone, Copy)]
pub struct ListPtr (NonNull<List>);

// identical to the ClassPtr implementation, however having them separate
// allows for the possibility of different implementations for the two
// types, which makes sense ig?
// also, they're only small haha and i cba
impl ListPtr {
  pub fn new(list : List) -> Self {
    Self(NonNull::new(Box::into_raw(Box::new(list))).unwrap())
  }

  pub fn get(&self) -> &List {
    unsafe { self.0.as_ref() }
  }
}

impl PartialEq for ListPtr {
  fn eq(&self, other: &Self) -> bool {
    eq(self.0.as_ptr(), other.0.as_ptr())
  }
}