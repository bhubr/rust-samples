use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Write;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_file_if_not_exists(file_name: &str) -> std::io::Result<()> {

  let file_path = Path::new(file_name);
  // https://doc.rust-lang.org/std/path/struct.Path.html#method.exists
  let file_exists = file_path.exists();
  if !file_exists {
    println!("File does not exists");
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
    let mut file = File::create(&file_name)?;
    file.write(b"name,email,phone")?;
  };
  Ok(())
}

fn main()-> std::io::Result<()> {
  let file_name = "address-book.csv";
  create_file_if_not_exists(&file_name);

  let mut v: Vec<_> = Vec::new();
  if let Ok(lines) = read_lines(file_name) {
    // Consumes the iterator, returns an (Optional) String
    let mut i = 0;
    for line in lines {
        if i > 0 {
            let d: Vec<_> = Ok(line).split(',').collect();
            v.push(d);
        }
        i += 1;
    }
  }

  Ok(())
}