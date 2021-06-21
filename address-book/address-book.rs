use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fmt;
use std::io;
use std::process::exit;

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

impl fmt::Debug for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Contact")
         .field("email", &self.email)
         .field("name", &self.name)
         .field("phone", &self.phone)
         .finish()
    }
}

fn read_csv_as_contacts_vector(file_name: &str) -> Vec<Contact>  {
  // https://doc.rust-lang.org/std/fs/struct.File.html#examples
  // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  // https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#read-lines-of-strings-from-a-file
  let file = File::open(&file_name);
  let file = match file {
      Ok(val) => val,
      _ => unreachable!()
  };
  let buf_reader = BufReader::new(file);
  // let mut contents = String::new();
  // buf_reader.read_to_string(&mut contents)?;
  let mut v: Vec<Contact> = Vec::new();
  for (num, line) in buf_reader.lines().enumerate() {
      // TODO check if line 0 contains name,email,phone
      if num > 0 {
          // let line = line?.clone();
          let line = match line {
              Ok(val) => val,
              _ => unreachable!()
          };
          let bits: Vec<&str> = line.split(',').collect();
          // println!("{:?}", bits);
          let c: Contact = Contact {
              name: bits[0].to_string(),
              email: bits[1].to_string(),
              phone: bits[2].to_string(),
          };
          v.push(c);
      }
  }
  v
}

fn main() -> std::io::Result<()> {
    let file_name = "address-book.csv";
    create_file_if_not_exists(&file_name);

    let mut v: Vec<Contact> = read_csv_as_contacts_vector(&file_name);
    // https://doc.rust-lang.org/book/ch13-02-iterators.html

    while true {

        let mut cmd = String::new();
        println!("Enter command:\n   c -> Show contacts\n   q -> Quit\n <x> -> Edit contact <x>");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read command");
        cmd = cmd.trim().to_string();

        // https://turreta.com/2019/09/07/rust-how-to-compare-strings/
        if cmd.eq("c") {
          println!(">> Showing contacts");
          for c in v.iter_mut() {
            println!("{:?}", c);
          }
        } else if cmd.eq("q") {
          println!(">> Quit");
          exit(0);
        } else {
          let contact_idx: usize = cmd.parse().unwrap();
          if contact_idx > v.len() {
            println!(">> Index {} is too big", contact_idx);
          } else {
            println!(">> Show contact {} -> {:?}", contact_idx, v[contact_idx]);
            let mut subcmd = String::new();
            println!("Enter subcommand:\n   n -> Edit name   \n   e -> Edit email\n   p -> Edit phone");
            io::stdin()
                .read_line(&mut subcmd)
                .expect("Failed to read subcommand");
            subcmd = subcmd.trim().to_string();

            let mut value = String::new();
            println!("Enter value:");
            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read value");
                value = value.trim().to_string();
            if subcmd.eq("n") {
              v[contact_idx].name = value;
            } else if subcmd.eq("e") {
              v[contact_idx].email = value;
            } else if subcmd.eq("p") {
              v[contact_idx].phone = value;
            }
          }
        }
    }


    Ok(())
}
