// Source: Classic Computer Science Problems in Java (D. Kopec, Manning), section 1.1
// Topics: variables, constants, functions, recursion

// Naive (slow) implementation
fn fib(n: i32) -> i32 {
  if n < 2 {
    return n;
  }
  fib(n - 1) + fib(n - 2)
}

fn main() {
  const NUM: i32 = 8;
  let result = fib(NUM);
  println!("fib({}) = {}", NUM, result);
}