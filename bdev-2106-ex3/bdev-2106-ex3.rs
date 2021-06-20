use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let filename = "samples/input3.txt";

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
    // let mut grid = [[char; 10]; 20];

    let mut empty_idx = usize::MAX;
    let mut consec_lines = 0;
    // populate grid from top to bottom
    for y in 0..20 {
      let mut hash_count = 0;
      let mut empty_cell_idx = usize::MAX;
      for x in 0..10 {
        let cell = v[19 - y].chars().nth(x).unwrap();
        // println!("x, y: {},{}, cell: {}", y, x, cell);
        if cell == '#' {
          hash_count += 1;
        } else if empty_cell_idx < usize::MAX {
          continue;
        } else {
          empty_cell_idx = x;
        }
      }
      if hash_count == 9 && empty_cell_idx < usize::MAX {
        // println!("Found candidate row {} {}", empty_cell_idx, 19 - y);
        if empty_idx == empty_cell_idx {
          consec_lines += 1;
        } else {
          empty_idx = empty_cell_idx;
          consec_lines = 1;
        }
        if consec_lines == 4 {
          // println!("Found 4 consec rows {} {}", empty_idx, 19 - y);
          let inv_y = 19 - y;
          let mut free_above = true;
          for yy in 0..inv_y {
            let cell = v[yy].chars().nth(empty_idx).unwrap();
            if cell == '#' {
              // println!("Cell occupied at {} {}", yy, empty_idx);
              free_above = false;
            }
          }
          if free_above {
            break;
          }
        }
      } else {
        empty_idx = usize::MAX;
        consec_lines = 0;
      }
    }
    if consec_lines == 0 {
      println!("NOPE");
    } else if consec_lines == 4 {
      println!("BOOM {}", empty_idx + 1);
    }
}
