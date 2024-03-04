#[no_mangle]
pub extern "C" fn factorial(n: u64) -> u64 {
  (1..=n).product()
}
