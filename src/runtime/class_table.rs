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

  pub fn add_class(&mut self, namespace : String, class: Class) {
      self.classes.insert(namespace + "." + &class.name, ClassPtr::new(class));
  }

  pub fn add_classes(&mut self, namespace : String, classes : [Class; 4]) {
    for mut class in classes {
      self.add_class(namespace.to_string(), class);
    }
  }

  pub fn get_class(&self, namespace : String, name : String) -> Option<ClassPtr> {
    self.classes.get(&(namespace + "." + &name)).copied()
  }
}