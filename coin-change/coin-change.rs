// Source: ?
// Topics: loops, conditionals, vectors, hashmaps
use std::collections::HashMap;

fn main() {
  let coins: [u32; 5] = [100, 50, 10, 5, 1];
  let amount = 397;

  let mut map: HashMap<u32, u32> = HashMap::new();
  let mut remainder = amount;
  for coin in coins.iter() {
    let qty = remainder / coin;
    remainder -= qty * coin;
    map.insert(*coin, qty);
  }
  for (c, qty) in map {
    let word = if qty == 1 { "coin" } else { "coins" };
    println!("Found {} {} of {}", qty, word, c);
  }
}