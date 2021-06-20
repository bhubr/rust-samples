// Full MK8 Scoreboard App
// Topics: strings (builds on split-string), hashmaps
use std::io;
use std::vec::Vec;

// enum GrandPrixCup {
//   Mushroom,
//   Flower,
//   Star,
//   Special,
//   Shell,
//   Banana,
//   Leaf,
//   Lightning
// }

struct GrandPrix {
  cup: &'static str,
  positions: Vec<usize>,
  global_score: u32,
  global_position: u32
}

fn parse_positions(positions: &str) -> Vec<usize> {
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
  v
}

fn encode_positions(v: &Vec<usize>, sep: char) -> String {
  let mut positions = String::new();
  let max_index = v.len() - 1;
  for (idx, p) in v.iter().enumerate() {
    positions.push_str(&p.to_string());
    if idx < max_index {
      positions.push(sep);
    }
  }
  positions
}

fn compute_score(positions: &Vec<usize>) -> u32 {
  let points: [u32; 12] = [15, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
  let mut score = 0;
  for p in positions.iter() {
    score += points[p - 1];
  }
  score
}

fn encode_gp_as_csv(gp: &GrandPrix) -> String {
  let mut csv = String::new();
  csv.push_str(gp.cup);
  csv.push(',');
  csv.push_str(&encode_positions(&gp.positions, '|'));
  csv.push(',');
  csv.push_str(&gp.global_position.to_string());
  csv.push(',');
  csv.push_str(&gp.global_score.to_string());
  csv
}

fn encode_gp_as_json(gp: &GrandPrix) -> String {
  let mut json = String::new();
  json.push_str("{\"cup\":\"");
  json.push_str(gp.cup);
  json.push_str("\",\"positions\":[");
  json.push_str(&encode_positions(&gp.positions, ','));
  json.push_str("],\"global_position\":");
  json.push_str(&gp.global_position.to_string());
  json.push_str(",\"global_score\":");
  json.push_str(&gp.global_score.to_string());
  json.push_str("}");
  json
}

fn main() {
  // Create & populate vector of cups
  let mut cups: Vec<&str> = Vec::new();

  cups.push("Mushroom");
  cups.push("Flower");
  cups.push("Star");
  cups.push("Special");
  cups.push("Shell");
  cups.push("Banana");
  cups.push("Leaf");
  cups.push("Lightning");

  // Ask user to choose a cup
  println!("Choose a cup");
  for num in 0..cups.len() {
    println!("{}. {}", num + 1, cups[num]);
  }

  let mut cup = String::new();
  io::stdin()
    .read_line(&mut cup)
    .expect("Failed to read cup");
    let cup: usize = cup.trim().parse().unwrap();
    println!("cup: {}", cup);
    if cup > cups.len() {
      println!("Invalid cup number: {}", cup);
    }
  let cup_label = cups[cup];

  let mut positions = String::new();

  println!("Enter your Grand Prix positions, separated by commas, e.g. 1,4,10,2");
  io::stdin()
    .read_line(&mut positions)
    .expect("Failed to read positions");

  let v = parse_positions(&positions);
  let global_score = compute_score(&v);
  println!("Total: {}", global_score);

  println!("Enter your Grand Prix global position (1-12)");
  let mut global_position = String::new();
  io::stdin()
    .read_line(&mut global_position)
    .expect("Failed to read global position");
    let global_position: u32 = global_position.trim().parse().unwrap();

  let gp = GrandPrix {
    global_score,
    positions: v,
    cup: cup_label,
    global_position
  };

  let csv = encode_gp_as_csv(&gp);
  println!("csv: {}", csv);

  let json = encode_gp_as_json(&gp);
  println!("json: {}", json);
}