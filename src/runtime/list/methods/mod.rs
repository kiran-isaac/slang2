mod signature;

#[allow(dead_code)]
pub mod inbuilt;

pub use signature::Signature;
use crate::runtime::{Class, Value};
use crate::types::Type;

#[derive(Clone, PartialEq)]
pub struct Method {
  pub name: String,
  pub signature: Vec<Type>,
  pub of: Option<Type>,
  pub body : MethodBody
}

impl Method {
  pub fn new(name : String, signature : Vec<Type>, of : Option<Type>, body : MethodBody) -> Self {
    Self {
      name,
      signature,
      of,
      body
    }
  }

  pub fn inbuilt_from_func_pointer(name : &str, signature : Signature, f : fn(Value, Value) -> Value) {
    
  }
}

#[derive(Clone, PartialEq)]
pub enum MethodBody {
  Inbuilt(fn(Value, Value) -> Value),
  UserDefined
}