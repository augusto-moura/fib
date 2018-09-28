
fn tc_fib(n: u8, a: u32, b: u32) -> u32 {
  match n {
    0 => a,
    1 => b,
    _ => tc_fib(n - 1, b, a + b)
  }
}

fn fib(n: u8) -> u32 {
  tc_fib(n, 0, 1)
}

fn main() {
  // Because we are considering the first number in the sequence as 1 and the
  // implementation expects it as 0, just adding 1 to 46 to get the expected
  // 46th position
  println!("{}", fib(47));
}

