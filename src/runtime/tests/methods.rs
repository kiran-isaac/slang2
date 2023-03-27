use crate::types::Type;
use crate::runtime::list::methods::Signature;

#[test]
pub fn test() {
  let sig1 = Signature::new(vec!(Type::Int, Type::Bool, Type::Int));

  println!("{}", sig1.to_string());
}