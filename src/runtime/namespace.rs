use crate::runtime::symbol_table::SymbolTable;

pub struct Namespace {
  pub name: String,
  pub full_name: String,
  pub parent: Option<Box<Namespace>>,
  pub symbols: SymbolTable,
}