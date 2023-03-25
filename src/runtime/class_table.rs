use std::collections::HashMap;
use crate::runtime::list::class::{Class};

pub struct ClassTable {
  pub(crate) classes : HashMap<String, Class>
}

impl ClassTable {
  pub fn new() -> Self {
    Self {
      classes: HashMap::new()
    }
  }

  pub fn add_class(&mut self, namespace : String, class: Class) {
      self.classes.insert(namespace + "." + &class.name, class);
  }

  pub fn get_class(&self, namespace : String, name : String) -> Option<&Class> {
      self.classes.get(&(namespace + "." + &name))
  }
}