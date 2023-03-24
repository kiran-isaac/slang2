pub mod internal_list;
pub use internal_list::{List, Pattern};
use std::ptr::NonNull;

use std::vec::Vec;

/// A class that an object may be instantiated from.
pub struct Class{
  name: String,
  supers: Vec<ClassPtr>,
  pattern: Pattern
}

/// A pointer to a class. These are managed by the runtime.
pub struct ClassPtr(NonNull<Class>);