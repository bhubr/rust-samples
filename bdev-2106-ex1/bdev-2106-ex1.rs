use std::env;
use std::fs;

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

    let d: u32 = v[0].parse().unwrap();
    let l: u32 = v[1].parse().unwrap();
    let res = d + 5 * l;

    println!("{}", res);
}
