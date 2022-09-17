#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // 操作系统会将物理内存映射成虚拟地址空间，程序在启动时看到的虚拟地址空间是一块完整连续的内存。
  // 栈内存从高位地址向下增长，且栈内存分配是连续的，一般操作系统对栈内存大小是有限制的. 在Rust里，函数调用时会创建一个临时栈空间, 调用结束后 Rust 会让这个栈空间里的对象自动进入 Drop 流程，最后栈顶指针自动移动到上一个调用栈顶
  // 堆上内存则是从低位地址向上增长，堆内存通常只受物理内存限制，而且通常是不连续的，一般由程序员手动申请和释放的
  {
    fn foo(x: &str) -> String {
      // fn foo(x: &str) -> &str { // error: a does not live long enough
      let a = "Hello, ".to_string() + x;
      // &a;
      a
    }
    let b = foo("world");
    println!("{}", b);
  }

  {
    // #![feature(box_syntax, box_patterns)] // rust nightly(box) insteads of Box::new
    // fn main() {
    //   let boxed = Some(box 5);
    //   match boxed {
    //       Some(box unboxed) => println!("Some {}", unboxed),
    //       None => println!("None"),
    //   }
    // }

    // Vec<i32>
    // (stack)    (heap)
    // ┌──────┐   ┌───┐
    // │ vec1 │──→│ 1 │
    // └──────┘   ├───┤
    //            │ 2 │
    //            ├───┤
    //            │ 3 │
    //            ├───┤
    //            │ 4 │
    //            └───┘

    // Vec<Box<i32>>
    // (stack)    (heap)   ┌───┐
    // ┌──────┐   ┌───┐ ┌─→│ 1 │
    // │ vec2 │──→│   │─┘  └───┘
    // └──────┘   ├───┤    ┌───┐
    //            │   │───→│ 2 │
    //            ├───┤    └───┘
    //            │   │─┐  ┌───┐
    //            ├───┤ └─→│ 3 │
    //            │   │─┐  └───┘
    //            └───┘ │  ┌───┐
    //                  └─→│ 4 │
    //                     └───┘
  }

  Ok(())
}