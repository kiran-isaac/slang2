//int:
//   add
//     int -> float -> float
//     int -> int -> int
//   sub
//     int -> float -> float
//     int -> int -> int
//   mul
//     int -> float -> float
//     int -> int -> int
//   div
//     int -> float -> float
//     int -> int -> int
//   mod
//     int -> int -> int
//   eq
//     int -> int -> bool
//   lt
//     int -> int -> bool
//     int -> float -> bool
//   le
//     int -> int -> bool
//     int -> float -> bool
//   gt
//     int -> int -> bool
//     int -> float -> bool
//   ge
//     int -> int -> bool
//     int -> float -> bool
//   to
//     int -> float
//     int -> bool
//
// float:
//   add
//     float -> int -> float
//     float -> float -> float
//   sub
//     float -> int -> float
//     float -> float -> float
//   mul
//     float -> int -> float
//     float -> float -> float
//   div
//     float -> int -> float
//     float -> float -> float
//   mod
//     float -> float -> float
//   eq
//     float -> float -> bool
//   lt
//     float -> int -> bool
//     float -> float -> bool
//   le
//     float -> int -> bool
//     float -> float -> bool
//   gt
//     float -> int -> bool
//     float -> float -> bool
//   ge
//     float -> int -> bool
//     float -> float -> bool
//   to
//     float -> int
//     float -> bool
//
// bool:
//   and
//     bool -> bool -> bool
//   or
//     bool -> bool -> bool
//   not
//     bool -> bool
//   eq
//     bool -> bool -> bool
//
// char:
//   eq
//     char -> char -> bool
//   lt
//     char -> char -> bool
//   le
//     char -> char -> bool
//   gt
//     char -> char -> bool
//   ge
//     char -> char -> bool
//   to
//     char -> int
//     char -> bool
//     char -> float

use crate::runtime::Value;

fn unpack_as_int(x : Value) -> i64 {
  match x {
    Value::Int(x) => x,
    _ => panic!("Expected int")
  }
}

fn unpack_as_float(x : Value) -> f64 {
  match x {
    Value::Float(x) => x,
    _ => panic!("Expected float")
  }
}

fn unpack_as_bool(x : Value) -> bool {
  match x {
    Value::Bool(x) => x,
    _ => panic!("Expected bool")
  }
}

fn unpack_as_char(x : Value) -> char {
  match x {
    Value::Char(x) => x,
    _ => panic!("Expected char")
  }
}

pub fn add_int_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Float(x as f64 + y)
}

pub fn add_int_int_int(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Int(x + y)
}

pub fn sub_int_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Float(x as f64 - y)
}

pub fn sub_int_int_int(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Int(x - y)
}

pub fn mul_int_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Float(x as f64 * y)
}

pub fn mul_int_int_int(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Int(x * y)
}

pub fn div_int_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Float(x as f64 / y)
}

pub fn div_int_int_int(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Int(x / y)
}

pub fn mod_int_int_int(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Int(x % y)
}

pub fn eq_int_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Bool(x == y)
}

pub fn lt_int_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Bool(x < y)
}

pub fn lt_int_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Bool((x as f64) < y)
}

pub fn le_int_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Bool(x <= y)
}

pub fn le_int_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Bool((x as f64) <= y)
}

pub fn gt_int_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Bool(x > y)
}

pub fn gt_int_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Bool((x as f64) > y)
}

pub fn ge_int_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_int(y);
  Value::Bool(x >= y)
}

pub fn ge_int_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_int(x);
  let y = unpack_as_float(y);
  Value::Bool((x as f64) >= y)
}

pub fn to_int_bool(x: Value) -> Value {
  let x = unpack_as_int(x);
  Value::Bool(x != 0)
}

pub fn to_int_float(x: Value) -> Value {
  let x = unpack_as_int(x);
  Value::Float(x as f64)
}

pub fn add_float_int_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Float(x + y as f64)
}

pub fn add_float_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Float(x + y)
}

pub fn sub_float_int_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Float(x - y as f64)
}

pub fn sub_float_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Float(x - y)
}

pub fn mul_float_int_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Float(x * y as f64)
}

pub fn mul_float_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Float(x * y)
}

pub fn div_float_int_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Float(x / y as f64)
}

pub fn div_float_float_float(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Float(x / y)
}

pub fn eq_float_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Bool(x == y)
}

pub fn lt_float_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Bool(x < y)
}

pub fn lt_float_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Bool(x < y as f64)
}

pub fn le_float_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Bool(x <= y)
}

pub fn le_float_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Bool(x <= y as f64)
}

pub fn gt_float_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Bool(x > y)
}

pub fn gt_float_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Bool(x > y as f64)
}

pub fn ge_float_float_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_float(y);
  Value::Bool(x >= y)
}

pub fn ge_float_int_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_float(x);
  let y = unpack_as_int(y);
  Value::Bool(x >= y as f64)
}

pub fn to_float_bool(x: Value) -> Value {
  let x = unpack_as_float(x);
  Value::Bool(x != 0.0)
}

pub fn to_float_int(x: Value) -> Value {
  let x = unpack_as_float(x);
  Value::Int(x as i64)
}

pub fn and_bool_bool_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_bool(x);
  let y = unpack_as_bool(y);
  Value::Bool(x && y)
}

pub fn or_bool_bool_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_bool(x);
  let y = unpack_as_bool(y);
  Value::Bool(x || y)
}

pub fn not_bool_bool(x: Value) -> Value {
  let x = unpack_as_bool(x);
  Value::Bool(!x)
}

pub fn eq_bool_bool_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_bool(x);
  let y = unpack_as_bool(y);
  Value::Bool(x == y)
}

pub fn eq_char_char_bool(x: Value, y: Value) -> Value {
  let x = unpack_as_char(x);
  let y = unpack_as_char(y);
  Value::Bool(x == y)
}

// TODO
// pub fn to_char_int(x: Value) -> Value {
//   let x = unpack_as_int(x);
//   Value::Char(x as char)
// }
