use crate::runtime::error::Error;
use crate::runtime::list::*;
use crate::types::Type;

fn slice_test_list() -> Result<List, Error> {
  let mut l1 = List::new_repeating_of(vec!(Type::Int, Type::Bool));

  l1.push(Value::Int(0))?;
  l1.push(Value::Bool(true))?;
  l1.push(Value::Int(1))?;
  l1.push(Value::Bool(false))?;
  l1.push(Value::Int(2))?;
  l1.push(Value::Bool(true))?;
  l1.push(Value::Int(3))?;
  l1.push(Value::Bool(false))?;
  Ok(l1)
}

#[test]
pub fn slice_none() -> Result<(), Error> {
  let l1 = slice_test_list()?;

  let l2 = l1.slice(None, None)?;
  Ok(())
}

#[test]
pub fn slice_some() -> Result<(), Error> {
  let l1 = slice_test_list()?;

  let l2 = l1.slice(Some(2), Some(6))?;
  assert_eq!(l2.to_string(), "[1, false, 2, true]");
  Ok(())
}

#[test]
pub fn slice_negative_none()-> Result<(), Error> {
  let l1 = slice_test_list()?;

  let l2 = l1.slice(Some(-2), None)?;
  assert_eq!(l2.to_string(), "[3, false]");
  Ok(())
}