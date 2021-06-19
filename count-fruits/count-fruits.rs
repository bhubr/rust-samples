// Source: https://github.com/WildCodeSchool/suite01/blob/master/basics/ex10.js (simplified)
// Topics: loops, conditionals, vectors, hashmaps
// Help: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
use std::collections::HashMap;

fn main() {
  let fruits = ["orange", "orange", "kiwi", "pineapple", "kiwi", "pineapple", "banana", "plum"];
  let mut map: HashMap<&'static str, i32> = HashMap::new();
  for fruit in fruits.iter() {
    // println!("{}", fruit);
    if map.get(fruit).is_some() {
      map.insert(fruit, map.get(fruit).unwrap() + 1);
    } else {
      map.insert(fruit, 1);
    }
  }
  for (key, value) in map {
    let word = if value == 1 { "time" } else { "times" };
    println!("'{}' was found {} {}", key, value, word);
  }
}