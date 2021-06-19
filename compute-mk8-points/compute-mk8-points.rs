// Compute Mario Kart 8 Grand Prix score, from positions per race
// Topics: strings (builds on split-string), hashmaps
use std::io;

fn main() {
  let mut positions = String::new();

  println!("Enter your Grand Prix positions, separated by commas, e.g. 1,4,10,2");
  io::stdin()
    .read_line(&mut positions)
    .expect("Failed to read positions");

  let mut v: Vec<usize> = Vec::new();
  let mut pos = String::new();
  for c in positions.chars() {
    if c == ',' {
      let posi = pos.parse().unwrap();
      v.push(posi);
      pos.clear();
    } else {
      pos.insert(pos.len(), c);
    }
  }
  // Handle last word
  if pos.len() > 0 {
    let posi = pos.trim().parse().unwrap();
    v.push(posi);
  }

  let points: [u32; 12] = [15, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
  let mut sum = 0;
  for p in v.iter() {
    println!("p {}", p);
    sum += points[p - 1];
  }

  println!("Total: {}", sum);

}