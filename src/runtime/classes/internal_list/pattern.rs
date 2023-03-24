use crate::types::Type;

/// A pattern that a list may follow.
pub struct Pattern {
  types : Vec<Type>,
  only : bool
}