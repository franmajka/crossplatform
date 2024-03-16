extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn factorial(n: u64) -> u64 {
  (1..=n).product()
}

#[no_mangle]
pub extern "C" fn sum(a: f32, b: f64) -> f64 {
  (a as f64) + b
}

#[no_mangle]
pub extern "C" fn getHello(s: *const c_char) -> *mut c_char {
  let s = unsafe { CStr::from_ptr(s).to_string_lossy() };
  CString::new(format!("Hello, {}", s)).unwrap().into_raw()
}
