/// A value that can be stored on a list.
pub enum Value{
  Class,
  Int(i64),
  Float(f64),
  Bool(bool)
}