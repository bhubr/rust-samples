fn get_char_at(inp: &str, pos: usize) -> char {
  inp.chars().nth(pos).unwrap()
}

fn main() {
  let inp = "Hello, world!";
  let pos1 = 3;
  let pos2 = 12;
  let ch1: char = get_char_at(inp, pos1);
  let ch2: char = get_char_at(inp, pos2);
  println!("Char at position {} in {} is {}", pos1, inp, ch1);
  println!("Char at position {} in {} is {}", pos2, inp, ch2);
}