#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case, unnameable_test_items)]

//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```
/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
  a + 2
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
     assert_eq!(4, add_two(2));
  }
}

fn main() -> Result<(), std::io::Error> {  
  {
    // Rust 的测试特性按精细度划分，分为 3 个层次：
    //    函数级；
    //    模块级；
    //    工程级；
    // 另外，Rust 还支持对文档进行测试。
    // cargo test

    // 函数级测试
    //
    //  #[test]
    //  fn it_works() {
    //    // do test work
    //    // assert!(expr)               测试表达式是否为 true 或 false
    //    // assert_eq!(expr, expr)      测试两个表达式的结果是否相等
    //  }
    //
    // 如果你的测试函数没完成，或没有更新，或是故意让它崩溃，但为了让测试能够顺利完成，我们主动可以给测试函数加上 #[should_panic] 标识，就不会让 cargo test 报错了。如
    //  #[test]
    //  #[should_panic]
    //  fn it_works() {
    //      assert!(false);
    //  }
    // 
    // 有时候，某个测试函数非常耗时，或暂时没更新，我们想不让它参与测试，但是又不想删除它，这时， #[ignore] 就派上用场了。
    //  #[test]
    //  #[ignore]
    //  fn expensive_test() {
    //      // code that takes an hour to run
    //  }

    // 模块级测试
    // 
    // 有时，我们会组织一批测试用例，这时，模块化的组织结构就有助于建立结构性的测试体系。Rust 中，可以类似如下写法：
    //   pub fn add_two(a: i32) -> i32 {
    //       a + 2
    //   }
    //   #[cfg(test)]
    //   mod tests {
    //       use super::add_two;
    //       #[test]
    //       fn it_works() {
    //           assert_eq!(4, add_two(2));
    //       }
    //   }
    // 也即在 mod 的上面写上 #[cfg(test)] ，表明这个模块是个测试模块。一个测试模块中，可以包含若干测试函数，测试模块中还可以继续包含测试模块，即模块的嵌套。
    // 如此，就形式了结构化的测试体系，甚是方便。

    // 工程级测试
    //
    // 函数级和模块级的测试，代码是与要测试的模块（编译单元）写在相同的文件中，一般做的是白盒测试。工程级的测试，一般做的就是黑盒集成测试了。
    // 我们在 tests 目录下，建立一个文件 testit.rs ，名字随便取皆可。内容为：
    //   extern crate adder;
    //   #[test]
    //   fn it_works() {
    //       assert_eq!(4, adder::add_two(2));
    //   }
    // 这里，比如，我们 src 中，写了一个库，提供了一个 add_two 函数，现在进行集成测试。
    // 首先，用 extern crate 的方式，引入这个库，由于是同一个项目，cargo 会自动找。引入后，就按模块的使用方法调用就行了，其它的测试标识与前面相同。

    // 文档级测试
    //
    // Rust 对文档的哲学，是不要单独写文档，一是代码本身是文档，二是代码的注释就是文档。Rust 不但可以自动抽取代码中的文档，形成标准形式的文档集合，还可以对文档中的示例代码进行测试。如本文件开头的文档测试
  }

  {
    // 性能测试: cargo bench
    //
    // 单元测试是用来校验程序的正确性的，然而，程序能正常运行后，往往还需要测试程序（一部分）的执行速度，这时，f就需要用到性能测试。
    // 通常来讲，所谓性能测试，指的是测量程序运行的速度，即运行一次要多少时间（通常是执行多次求平均值）。Rust 竟然连这个特性都集成在语言基础特性中，真的是一门很重视工程性的语言。
    //    #[bench]
    //    fn bench_add_two(b: &mut Bencher) {
    //        b.iter(|| add_two(2));
    //    }

    // 写测评代码的时候，需要注意以下一些点：
    //   只把你需要做性能测试的代码（函数）放在评测函数中；
    //   对于参与做性能测试的代码（函数），要求每次测试做同样的事情，不要做累积和改变外部状态的操作；
    //   参数性能测试的代码（函数），执行时间不要太长。太长的话，最好分成几个部分测试。这也方便找出性能瓶颈所在地方。
  }

  Ok(())
}