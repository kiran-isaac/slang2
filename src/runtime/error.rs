use std::fmt::{Debug};

#[derive(Debug)]
pub enum Error {
  TypeError(String),
  SliceError(String)
}