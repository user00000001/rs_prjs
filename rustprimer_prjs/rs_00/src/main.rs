#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  {
    // macro_rules! macro_name { macro_body(pattern => do_something) }
    println!("Hello, World!"); // ()/[]/{} are equal
    println!["Hello, World!"];
    println!{"Hello, World!"};

    // create_function!{bar}; // macro defination first

    macro_rules! create_function {
      ($func_name: ident) => { // ident: macro type
        fn $func_name() {
          println!("function {:?} is called", stringify!($func_name));
        }
      };
    }
    create_function!(foo);
    // fn foo() {
    //     println!("function {:?} is called", stringify!(foo))
    // }
    foo();

    // macro name, function name not in the same namespace.
    fn foo1(x: i32) -> i32 {
      x * x
    }
    macro_rules! foo1 {
      ($x: ident) => {
        println!("{:?}", $x);
      };
    }
    let a = 5;
    foo1!(a); // foo1: macro
    println!("{}", foo1(a)); // foo1: function
  }

  {
    // designator: macro type
    // 
    // ident: 标识符，用来表示函数或变量名
    // expr: 表达式
    // block: 代码块，用花括号包起来的多个语句
    // pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
    // path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
    // tt: 单个语法树
    // ty: 类型，语义层面的类型，如 i32, char
    // item: 条目，
    // meta: 元条目
    // stmt: 单条语句，如 let a = 42;

    // repetition: $(...)* | $(...)+ | $(...),* | $(...);+ ...
    // 
    macro_rules! vector {
      {$($x:expr),*} => [ // ()/[]/{} are the same(two sides of "=>").
        {
          let mut temp_vec = Vec::new();
          $(temp_vec.push($x);)*
          temp_vec
        }
      ];
    }
    let a = vector![1,2,4,8];
    println!("{:?}", a);
    // vector![3, x*x, s-t];
    // {
    //   let mut temp_vec = Vec::new();
    //   temp_vec.push(3);
    //   temp_vec.push(x*x);
    //   temp_vec.push(s-t);
    //   temp_vec
    // }

    // recursion: 
    // 
    macro_rules! find_min {
      ($x:expr) => { // pattern 1
        ($x)
      };
      ($x:expr, $($y:expr),+) => { // pattern 2
        std::cmp::min($x, find_min!($($y),+)) // using self
      }
    }
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32+2, 2u32));
    println!("{}", find_min!(5u32, 2u32*3, 4u32));
    // 1, std::cmp::min(5u32, find_min!(2u32 * 3, 4u32))
    // 2, std::cmp::min(5u32, std::cmp::min(2u32 * 3, find_min!(4u32)))
    // 3, std::cmp::min(5u32, std::cmp::min(2u32 * 3, 4u32))

    // hygienic Macro:
    //
    macro_rules! foo {
      () => {
        let x = 3;
      };
    }
    macro_rules! bar {
      ($v:ident) => {
        let $v = 3;
      };
    }
    foo!();
    // println!("{}", x); // error: variable x is out of the scope of macro defination
    bar!(a); // a is as an input parameter
    println!("{}", a);

    macro_rules! foo1 {
      () => {
        fn x() {}
      };
    }
    foo1!();
    x(); // item x has some differences with variable
  }

  {
    // macro import/export: #[macro_use]/#[macro_export]
    //
    // in foo crate
    macro_rules! m1 { () => (()) }
    // 宏 m1 在这里可用
    mod foo {
        // 宏 m1 在这里可用
        #[macro_export]
        macro_rules! m2 { () => (()) }
        // 宏 m1 和 m2 在这里可用
    }
    // 宏 m1 在这里可用
    #[macro_export]
    macro_rules! m3 { () => (()) }
    // 宏 m1 和 m3 在这里可用
    #[macro_use]
    mod bar {
        // 宏 m1 和 m3 在这里可用
        macro_rules! m4 { () => (()) }
        // 宏 m1, m3, m4 在这里均可用
    }
    // 宏 m1, m3, m4 均可用

    // #[macro_use]
    // extern crate foo;
    // // foo 中 m2, m3 都被导入
    // 
    // #[macro_use(m3)]
    // extern crate foo;
    // // foo 中只有 m3 被导入

    // macro: debug
    //
    // rustc -Z unstable-options --pretty=expanded hello.rs
    // cargo rustc -- -Z unstable-options --pretty=expanded
  }

  Ok(())
}