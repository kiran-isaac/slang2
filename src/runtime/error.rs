use std::process::exit;

pub enum Error {
  TypeError(String)
}

impl Error {
  pub fn throw(&self) {
    match self {
      Error::TypeError(e) => {
        println!("Type Error: {}", e);
      }
    }
    exit(1);
  }
}