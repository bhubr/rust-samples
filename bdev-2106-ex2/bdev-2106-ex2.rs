use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let filename = "samples/input4.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut v: Vec<String> = Vec::new();
    let mut word = String::new();

    // Iterate string characters
    for c in contents.chars() {
        if c == '\n' {
            v.push(word.clone());
            word.clear();
        } else {
            word.insert(word.len(), c);
        }
    }
    if word.len() > 0 {
        v.push(word.clone());
    }

    // --- begin ---
    let mut map: HashMap<&String, i32> = HashMap::new();

    for button in v.iter() {
      if map.get(button).is_some() {
        map.insert(button, map.get(button).unwrap() + 1);
      } else {
        map.insert(button, 1);
      }
    }

    for (k, v) in map {
      if v == 2 {
        println!("{}", k);
      }
    }
}
