mod slice;

use crate::runtime::error::Error;
use crate::runtime::list::*;
use crate::types::Type;

#[test]
pub fn take_frames_and_reverse() -> Result<(), Error>{
  let mut is_even = List::new_repeating_of(vec!(Type::Int, Type::Bool));

  is_even.push(Value::Int(0))?;
  is_even.push(Value::Bool(true))?;
  is_even.push(Value::Int(1))?;
  is_even.push(Value::Bool(false))?;

  let mut is_even_reversed = List::new_repeating_of(vec!(Type::Int, Type::Bool));
  is_even_reversed.take_end_segment(&mut is_even)?;
  is_even_reversed.take_end_segment(&mut is_even)?;

  assert_eq!(is_even.to_string(), "[]");
  assert_eq!(is_even_reversed.to_string(), "[1, false, 0, true]");
  Ok(())
}

#[test]
pub fn recursive_list_structure() -> Result<(), Error> {
  let mut container = List::new_repeating_of(vec!());

  let mut is_even = List::new_repeating_of(vec!(Type::Int, Type::Bool));

  // Reusing the ones from the last test bc why not
  is_even.push(Value::Int(0))?;
  is_even.push(Value::Bool(true))?;
  is_even.push(Value::Int(1))?;
  is_even.push(Value::Bool(false))?;

  let mut is_even_reversed = List::new_repeating_of(vec!(Type::Int, Type::Bool));
  is_even_reversed.take_end_segment(&mut is_even)?;

  container.push(Value::List(Box::new(is_even)))?;
  container.push(Value::List(Box::new(is_even_reversed)))?;

  assert_eq!(container.to_string(), "[[0, true], [1, false]]");
  Ok(())
}