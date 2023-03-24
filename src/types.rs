use crate::runtime::list::ClassPtr;

/// A type that a value may have. Either an object w/ a class, a primitive list, or a primitive type.
pub enum Type{
  Class(ClassPtr),
  Char,
  Int, 
  Float, 
  Bool
}