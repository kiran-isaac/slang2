use crate::runtime::symtab::SymTab;

#[allow(dead_code)]
pub struct Namespace {
  pub name: String,
  pub full_name: String,
  pub parent: Option<Box<Namespace>>,
  pub symbols: SymTab,
}

impl Namespace {
  pub fn new(name: String, parent: Option<Box<Namespace>>) -> Self {
    Self {
      name : name.to_string(),
      full_name: match &parent {
        Some(p) => p.full_name.clone() + "." + &name,
        None => name.clone(),
      },
      parent,
      symbols : SymTab::new()
    }
  }
}