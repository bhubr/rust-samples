use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

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

struct Contact {
    name: String,
    email: String,
    phone: String,
}

fn main() -> std::io::Result<()> {
    let file_name = "address-book.csv";
    create_file_if_not_exists(&file_name);

    // https://doc.rust-lang.org/std/fs/struct.File.html#examples
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#read-lines-of-strings-from-a-file
    let file = File::open(&file_name)?;
    let buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents)?;
    let mut v: Vec<Contact> = Vec::new();
    for (num, line) in buf_reader.lines().enumerate() {
        // TODO check if line 0 contains name,email,phone
        if num > 0 {
            let line = line?.clone();
            let bits: Vec<&str> = line.split(',').collect();
            println!("{:?}", bits);
            let c: Contact = Contact {
                name: bits[0].to_string(),
                email: bits[1].to_string(),
                phone: bits[2].to_string(),
            };
            v.push(c);
        }
    }
    Ok(())
}
