mod signature;

pub use signature::Signature;

#[derive(Clone, PartialEq)]
pub struct Method {
  pub name: String,
  pub signature: Signature,
}