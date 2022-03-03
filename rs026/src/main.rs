use std::env;
use std::process;

pub mod lib;
use lib::Config;
use lib as mylib;

// CASE_INSENSITIVE=1 cargo run Body poem.txt > output.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = Config::new(&args[..]);
    match result {
        Ok(config) => {
            // mylib::run(config).unwrap_or_else(|err| {
            //     panic!("{}", err);
            // });
            if let Err(err) = mylib::run(config) {
                // println!("ERROR: {}", err);
                eprintln!("Problem parsing arguments: {}", err);
                process::exit(1);
            }
        }
        Err(err) => {
            // println!("{}", err);
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    }  
}
