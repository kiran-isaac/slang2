use std::collections::HashMap;
use crate::runtime::list::class::{ClassPtr, Class};

pub struct ClassTable {
  pub(crate) classes : HashMap<String, ClassPtr>
}

impl ClassTable {
  pub fn new() -> Self {
    Self {
      classes: HashMap::new()
    }
  }

  pub fn add_class(&mut self, namespace : String, class: &mut Class) {
      self.classes.insert(namespace + "." + &class.name, ClassPtr::new(class));
  }

  pub fn get_class(&self, namespace : String, name : String) -> Option<ClassPtr> {
    self.classes.get(&(namespace + "." + &name)).copied()
  }
}