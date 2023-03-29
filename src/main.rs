mod runtime;
mod parser;
mod types;

use std::env;

use runtime::symtab::SymTab;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args[1] == "run" {
    let symtab = SymTab::new();
  }
}