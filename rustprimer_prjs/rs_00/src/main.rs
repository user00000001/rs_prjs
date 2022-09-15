#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

use std::env;
use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};

fn main() -> Result<(), io::Error> {
  {
    // Read Trait: read
    // Write Trait: write/flush

    // BufRead Trait: read_line/lines
    //    io::BufReader::new(reader)
    //    io::BufReader::with_capacity(bufSize, reader)

    // BufWriter: write/flush
    //    io::BufWriter::new(writer)
    //    io::BufWriter::with_capacity(bufSize, writer)

    fn read_from_stdin(buf: &mut String) -> io::Result<()> {
      r#try!(io::stdin().read_line(buf));
      Ok(())
    }
    fn write_to_stdout(buf: &[u8]) -> io::Result<()> {
      r#try!(io::stdout().write(&buf));
      Ok(())
    }
    let mut input = String::new();
    r#try!(io::stdin().read_line(&mut input));
    println!("You typed: {}", input.trim());
  }

  {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().unwrap();
    let num = input.trim().parse::<i32>().unwrap();
    println!("您输入的数字是：{}", num);

    macro_rules! numin {
          () =>{
              {
                let mut input = String::new();
                  std::io::stdin()
                      .read_line(&mut input)
                    .expect("Failed to read line");
                  input.trim().parse().unwrap()
            }
        };
    }
    let num: i32 = numin!();
    println!("您输入的数字是：{}", num);

    let args = env::args();
    for arg in args {
      println!("{}", arg);
    }
  }

  {
    print!("hello!\ninput:"); // print! not flush to output
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    println!("line: {}", input);

    // macro_rules! print {
    //   ($($arg:tt)*) => { ... };
    // }
    macro_rules! printf {
      ($($arg:tt)*) =>{
        print!($($arg)*);
        io::stdout().flush().unwrap(); // flush at every invoke
      }
    }
    printf!("hello!\ninput:"); // print! not flush to output
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    println!("line: {}", input);
  }

  {
    // write buf to filename
    fn create_file(filename: &str, buf: &[u8]) -> io::Result<()> {
      let mut f = r#try!(File::create(filename));
      r#try!(f.write((&buf)));
      Ok(())
    }
    // read from filename to buf
    fn read_file(filename: &str, buf: &mut String) -> io::Result<()> {
      let mut f = r#try!(File::open(filename));
      r#try!(f.read_to_string(buf));
      Ok(())
    }
    let f = "../../target/foo.txt";
    let mut buf = String::new();
    match create_file(f, b"Hello, World!") {
      Ok(()) => {
        match read_file(f, &mut buf) {
          Ok(()) => {
            println!("{}", buf);
          },
          Err(err) => {
            println!("{}", err);
          }
        }
      }
      Err(err) => {
        println!("{}", err);
      }
    }
    let file = OpenOptions::new().write(true).truncate(true).open("../../target/foo.txt"); // chain method instead of input open options
  }

  return Ok(())
}