// Topics: strings, loops, vectors

fn main() {
  let languages = "Rust,Go,C++,Java";
  let mut v: Vec<String> = Vec::new();
  let mut word = String::new();

  // Iterate string characters
  for c in languages.chars() {
    if c == ',' {
      v.push(word.clone());
      word.clear();
    } else {
      word.insert(word.len(), c);
    }
  }
  if word.len() > 0 {
    v.push(word.clone());
  }

  for w in v.iter() {
    println!("{}", w);
  }
}