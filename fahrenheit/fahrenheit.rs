// Topics: variables, loops, constants

fn main() {
  let lower = 0;
  let upper = 300;
  let step = 20;
  let mut fahr = lower;
  let mut celsius;

  while fahr <= upper {
    celsius = 5 * (fahr - 32) / 9;
    println!("{}\t{}", fahr, celsius);
    fahr += step;
  }
}