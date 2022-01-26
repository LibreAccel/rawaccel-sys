#![feature(extern_types)]

#[cxx::bridge(namespace = "rawaccel")]
pub mod utility {
  unsafe extern "C++" {
    include!("rawaccel-sys/rawaccel/common/utility.hpp");

    pub fn minsd(a: f64, b: f64) -> f64;
    pub fn maxsd(a: f64, b: f64) -> f64;
    pub fn clampsd(v: f64, lo: f64, hi: f64) -> f64;
    pub fn ilogb(x: f64) -> i32;
    pub fn scalbn(x: f64, n: i32) -> f64;
    pub fn infnan(x: f64) -> bool;
  }
}

#[cfg(test)]
mod test {
  use crate::utility;

  #[test]
  fn test_utility_minsd() { assert_eq!(utility::minsd(2.0, 3.0), 2.0) }

  #[test]
  fn test_utility_maxsd() { assert_eq!(utility::maxsd(2.0, 3.0), 3.0) }
}
