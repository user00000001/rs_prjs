#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    fn guess(n: i32) -> bool {
      if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
      }
      n == 5
    }
    // guess(11); // panic, thread or process exits.
    guess(2);

    // enum Option<T> {
    //   None,
    //   Some(T),
    // }
    fn find(haystack: &str, needle: char) -> Option<usize> {
      for (offset, c) in haystack.char_indices() {
        if c == needle {
          return Some(offset);
        }
      }
      None
    }
    let file_name = "foobar.rs";
    match find(file_name, '.') {
      Some(i) => println!("File extension: {}", &file_name[i+1..]),
      None => println!("No file extension found."),
    }

    // impl<T> Option<T> {
    //   fn unwrap(self) -> T {
    //     match self {
    //       Option::Some(val) => val,
    //       Option::None =>
    //         panic!("called `Option::unwrap()` on a `None` value"),
    //     }
    //   }
    // }
    println!("{}", find(file_name, '.').unwrap());

    fn extension_explicit(file_name: &str) -> Option<&str> {
      match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
      }
    }
    match extension_explicit("foo.rs") {
      None => println!("no extention"),
      Some(ext) => assert_eq!(ext, "rs"),
    }

    // fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    //     match option {
    //         None => None,
    //         Some(value) => Some(f(value)),
    //     }
    // }
    fn extension(file_name: &str) -> Option<&str> {
      find(file_name, '.').map(|i|&file_name[i+1..]) // map is used to handle Option here
    }
    match extension("foo.rs") {
      None => println!("no extention"),
      Some(ext) => assert_eq!(ext, "rs"),
    }

    // fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    //     match option {
    //         None => default,
    //         Some(value) => value,
    //     }
    // }
    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs"); // unwrap_or: provide a default value
    assert_eq!(extension("foo").unwrap_or("rs"), "rs");

    // fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
    //     where F: FnOnce(T) -> Option<A> {
    //     match option {
    //         None => None,
    //         Some(value) => f(value),
    //     }
    // }

    use std::path::Path;
    fn file_name_fn(file_path: &str) -> Option<&str> {
      let path = Path::new(file_path);
      if let Some(fn_) = path.file_name() {
        return fn_.to_str()
      }
      None
    }
    fn file_path_ext(file_path: &str) -> Option<&str> {
      file_name_fn(file_path).and_then(extension) // and_then: simlar to map above
    }
    assert_eq!(file_path_ext("foo.rs"), Some("rs"));
    assert_eq!(file_path_ext("foo"), None);
  }

  {
    // Result
    // 
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //
    // type Option<T> = Result<T, ()>;
    //
    // impl<T, E: ::std::fmt::Debug> Result<T, E> {
    //     fn unwrap(self) -> T {
    //         match self {
    //             Result::Ok(val) => val,
    //             Result::Err(err) =>
    //               panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
    //         }
    //     }
    // }

    fn double_number(number_str: &str) -> i32 {
      2 * number_str.parse::<i32>().unwrap()
    }
    let n: i32 = double_number("10");
    assert_eq!(n, 20);

    // impl str {
    //   fn parse<F: FromStr>(&self) -> Result<F, F::Err>;
    // }
    // pub trait FromStr {
    //   type Err;
    //   fn from_str(s: &str) -> Result<Self, Self::Err>;
    // }
    // impl FromStr for i32 {
    //     type Err = ParseIntError;
    //     fn from_str(src: &str) -> Result<i32, ParseIntError> {
    //     }
    // }
    use std::num::ParseIntError;
    fn double_number1(number_str: &str) -> Result<i32, ParseIntError> {
      number_str.parse::<i32>().map(|n|n*2) // map handles value, then return value or return error 
    }
    match double_number1("10") {
      Ok(n) => assert_eq!(n, 20),
      Err(err) => println!("Error: {:?}", err),
    }
    // map_err
    // or_else

    use std::result;
    type Result1<T> = result::Result<T, ParseIntError>; // alias
    fn double_number2(number_str: &str) -> Result1<i32> {
      unimplemented!();
    }

    // fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> { // convert option to result
    //     match option {
    //         Some(val) => Ok(val),
    //         None => Err(err),
    //     }
    // }
    use std::env;
    fn double_arg(mut argv: env::Args) -> Result<i32, String> {
      argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err|err.to_string()))
        .map(|n|2*n)
    }
    match double_arg(env::args()) {
      Ok(n) => println!("{}", n),
      Err(err) => println!("Error: {}", err),
    }

    // echo "100" > ../../target/foobar
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
      File::open(file_path)
        .map_err(|err|err.to_string())
        .and_then(|mut file|{
          let mut contents = String::new();
          file.read_to_string(&mut contents)
            .map_err(|err|err.to_string())
            .map(|_|contents)
        })
        .and_then(|contents|{
          contents.trim().parse::<i32>()
            .map_err(|err|err.to_string())
        })
        .map(|n|2*n)
    }
    match file_double("../../target/foobar") {
      Ok(n) => println!("{}", n),
      Err(err) => println!("Error: {}", err),
    }

    // using match & if let instead
    fn file_double1<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
      let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
      };
      let mut contents = String::new();
      if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
      }
      let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
      };
      Ok(2*n)
    }
    match file_double1("../../target/foobar") {
      Ok(n) => println!("{}", n),
      Err(err) => println!("Error: {}", err),
    }

    // macro_rules! try { // using ? instead r#try! macro
    //   ($e:expr) => (match $e {
    //     Ok(val) => val,
    //     Err(err) => return Err(::std::convert::From::from(err)),
    //   });
    // }
    use std::error::Error;
    fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
      let mut file = r#try!(File::open(file_path));
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      let n = r#try!(contents.trim().parse::<i32>());
      Ok(2 * n)
    }
    match file_double2("../../target/foobar") {
      Ok(n) => println!("{}", n),
      Err(err) => println!("Error: {}", err),
    }

    // self-define Error: implements From trait for convert to self-define Error
    use std::io;
    use std::num;
    #[derive(Debug)]
    enum CliError {
      Io(io::Error),
      Parse(num::ParseIntError)
    }
    impl From<io::Error> for CliError {
      fn from(err: io::Error) -> CliError {
        CliError::Io(err)
      }
    }
    impl From<num::ParseIntError> for CliError {
      fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
      }
    }
    fn file_double_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
      let mut file = r#try!(File::open(file_path).map_err(CliError::Io));
      let mut contents = String::new();
      r#try!(file.read_to_string(&mut contents).map_err(CliError::Io));
      let n: i32 = r#try!(contents.trim().parse().map_err(CliError::Parse));
      Ok(n * 2)
    }
    match file_double2("../../target/foobar") {
      Ok(n) => println!("{}", n),
      Err(err) => println!("Error: {}", err),
    }
  }
}