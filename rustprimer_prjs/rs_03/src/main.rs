#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]

use std::io;

fn read_input() -> io::Result<()> {
    let mut input = String::new();
    r#try!(io::stdin().read_line(&mut input)); // you can use the `?` operator instead; you can still access the deprecated `try!()` macro using the "raw identifier" syntax
    io::stdin().read_line(&mut input)?;
    println!("You typed: {}", input.trim());
    Ok(())
}

fn main() {
    read_input();

    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("WTF!");
        println!("You typed: {}", input.trim());
    }
    
    {
        print!("this ");
        print!("will ");
        print!("be ");
        print!("on ");
        print!("the ");
        print!("same ");
        print!("line ");
        print!("this string has a newline, why not choose println! instead?\n");

        println!("hello there!");
        println!("format {} arguments", "some");
    }

    {
        print!("请输入一个字符串："); // not print first as expected
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取失败");
        print!("您输入的字符串是：{}\n", input);
    }

    {
        use std::io::Write;
        print!("请输入一个字符串：");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取失败");
        print!("您输入的字符串是：{}\n", input);
    }

    {
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;
        // 创建一个文件路径
        let path = Path::new("Cargo.toml");
        let display = path.display();
        // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
        let mut file = match File::open(&path) {
            // 处理打开文件可能潜在的错误
            Err(why) => panic!("couldn't open {}: {}", display,
                                                    Error::description(&why)),
            Ok(file) => file,
        };
        // 文件输入数据到字符串，并返回 `io::Result<usize>` 类型
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                    Error::description(&why)),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }
    }

    {
        // 输出文本
        static LOREM_IPSUM: &'static str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
        use std::error::Error;
        use std::io::prelude::*;
        use std::fs::File;
        use std::path::Path;
        let path = Path::new("../../target/lorem_ipsum.txt");
        let display = path.display();
        // 用只写模式打开一个文件，并返回 `io::Result<File>` 类型
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            display,
                            Error::description(&why)),
            Ok(file) => file,
        };
        // 写入 `LOREM_IPSUM` 字符串到文件中, 并返回 `io::Result<()>` 类型
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                Error::description(&why))
            },
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}