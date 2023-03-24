/// A type that a value may have. Either an object w/ a class, a primitive list, or a primitive type.
pub enum Type{
  Class(ClassPtr),
  List, Int, Float, Bool
}

/// A pattern that a list may follow.
pub struct Pattern {
  types : Vec<Type>,
  only : bool
}