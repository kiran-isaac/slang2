pub mod runtime;
pub mod parser;
mod types;

use runtime::list::{Class, Pattern};
use types::Type;

fn main() {
  let testPattern = Pattern {types : vec![Type::Int], only : false};
  let test = Class::new("test".to_string(), Vec::new(), testPattern);

  let testPattern2 = Pattern {types : vec![Type::Int], only : false};
  let test2 = Class::new("test".to_string(), Vec::new(), testPattern2);
}
