#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case, unnameable_test_items)]


fn main() -> Result<(), std::io::Error> {
  // Unsafe Rust 在 Safe Rust 的所有特性上，只给程序员开放了以下四种能力：
  //   对原始指针进行解引（Dereference raw pointers）；
  //   调用 unsafe 函数（包括 C 函数，内部函数，和原始分配器）；
  //   实现 unsafe traits；
  //   修改（全局）静态变量。

  // Rust 中定义的不确定性行为有如下一些：
  //   对空指针或悬挂指针进行解引用；
  //   读取未初始化的内存；
  //   破坏指针重命名规则（比如同一资源的 &mut 引用不能出现多次，&mut 与 & 不能同时出现）；
  //   产生无效的原生值：
  //     空指针，悬挂指针；
  //     bool 值不是 0 或 1；
  //     未定义的枚举取值；
  //     char 值超出取值范围 [0x0, 0xD7FF] 和 [0xE000, 0x10FFFF]；
  //     非 utf-8 字符串；
  //   Unwinding 到其它语言中；
  //   产生一个数据竞争。

  // 以下一些情况，Rust 认为不属于安全性的处理范畴，即认为它们是“安全”的：
  //   死锁；
  //   存在竞争条件；
  //   内存泄漏；
  //   调用析构函数失败；
  //   整数溢出；
  //   程序被中断；
  //   删除产品数据库（:D）；
  
  Ok(())
}