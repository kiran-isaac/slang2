mod signature;
mod method_ptr;

pub use signature::Signature;
pub use method_ptr::MethodPtr;

pub struct Method {
  pub name: String,
  pub signature: Signature,
}