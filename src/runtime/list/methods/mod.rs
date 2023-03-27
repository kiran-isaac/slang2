mod signature;

pub use signature::Signature;

pub struct Method {
  pub name: String,
  pub signature: Signature,
}