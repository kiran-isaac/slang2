mod signature;
pub mod inbuilt;

pub use signature::Signature;
use crate::runtime::{Class, Value};
use crate::types::Type;

#[derive(Clone, PartialEq)]
pub struct Method {
  pub name: String,
  pub signature: Vec<Type>,
  pub of: Option<Box<Class>>,
  pub body : MethodBody
}

impl Method {
  pub fn new(name : String, signature : Vec<Type>, of : Option<Box<Class>>, body : MethodBody) -> Self {
    Self {
      name,
      signature,
      of,
      body
    }
  }
}

#[derive(Clone, PartialEq)]
pub enum MethodBody {
  Inbuilt(fn(Value, Value) -> Value),
  UserDefined
}