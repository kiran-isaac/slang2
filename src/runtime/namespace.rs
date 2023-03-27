use crate::runtime::symtab::SymTab;

#[allow(dead_code)]
pub struct Namespace {
  pub name: String,
  pub full_name: String,
  pub parent: Option<Box<Namespace>>,
  pub symbols: SymTab,
}