use std::process::exit;

pub enum Error {
  TypeError(String)
}

pub fn throw(e : Error) {
  match e {
    Error::TypeError(e) => {
      println!("Type Error: {}", e);
    }
  }
  exit(1);
}