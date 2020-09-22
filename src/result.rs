pub use failure::*;


#[derive(Debug, Fail)]
pub enum ErrorType {
  #[fail(display = "invalid ldraw file: {}", _0)]
  Invalid(&'static str),
  #[fail(display = "malformed ldraw file: {}", _0)]
  Malformed(&'static str),
}

pub type Result<T> = StdResult<T, Error>;
pub use core::result::Result as StdResult;

#[macro_export]
macro_rules! err_invalid {
  ($msg:expr) => {{
    const ERR: self::ErrorType = self::ErrorType::Invalid($msg);
    ERR
  }};
}

#[macro_export]
macro_rules! err_malformed {
  ($msg:expr) => {{
    const ERR: self::ErrorType = self::ErrorType::Malformed($msg);
    ERR
  }};
}
