use std::error::Error as eError;
use std::fs::{ self, File, OpenOptions };
use std::io::{ ErrorKind, Write, Read, Error };

fn main() -> Result<(), Box<dyn eError>> {
    // let v = vec![1,2,3,4];
    // v[100]; // panic
    // panic!("crash and burn"); // panic

    let path = "src/main.rs.bak";
    let f = File::open(&path);
    match f {
        Ok(file) => println!("Found, {:?}", file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("Not Found, {}", err);
                match File::create(&path) {
                    Ok(mut file) => { 
                        println!("create file: {:?}", file);
                        match file.write("Hello, World!".as_bytes()) {
                            Ok(ret) => {
                                println!("{}", ret);
                                file.flush().expect("Wrong Flush!");
                            },
                            Err(err) => panic!("{}", err)
                        };
                    }
                    Err(err) => panic!("create file {} failed: {}", path, err),
                };
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
            
        },
    }
    File::open(&path).unwrap();
    File::open(&path).expect("Not Found");
    let mut f1 = OpenOptions::new()
        .append(true)
        .open("filename.bak").unwrap_or_else(|error| -> File {
        if error.kind() == ErrorKind::NotFound {
            File::create("filename.bak").unwrap_or_else(|error| -> File {
                panic!("Problem creating the file: {}", error);
            })
        } else {
            panic!("Problem opening the file: {}", error);
        }
    });
    f1.write("\nhello".as_bytes()).expect("Write Failed!");
    match File::open("filename.bak") {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s).expect("Read Failed!");
            println!("{}", s);
        }
        Err(error) => panic!("Open for Read Failed!{}", error)
    }
    println!("fn: {}", read_username_from_file(&String::from("filename.bak")).unwrap());
    println!("fn1: {}", read_username_from_file1(&String::from("filename.bak")).unwrap());
    println!("fn2: {}", read_username_from_file2(&String::from("filename.bak")).unwrap());
    File::open(&path)?;
    Ok(())
}

fn read_username_from_file(path: &String) -> Result<String, Error> {
    match File::open(path) {
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(error) => Err(error),
            }
        }
        Err(error) => {
            Err(error)
        }
    }
}

fn read_username_from_file1(path: &String) -> Result<String, Error> {
    let mut s = String::new();
    // let mut file = File::open(path)?;
    // file.read_to_string(&mut s)?;
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2(path: &String) -> Result<String, Error> {
    fs::read_to_string(path)
}