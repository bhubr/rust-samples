// Source: C Programming Language (K&R, Pearson), section 1.2
// Topics: variables, constants, loops

fn main() {
  const UPPER: i32 = 300;
  const LOWER: i32 = 0;
  const STEP: i32 = 20;
  let mut fahr = LOWER;
  let mut celsius;

  while fahr <= UPPER {
    celsius = 5 * (fahr - 32) / 9;
    println!("{}\t{}", fahr, celsius);
    fahr += STEP;
  }
}