#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // Send / Sync
  {
    // included in std::marker, relative to threads safety, a kind of contraint, no method defined, no associated items

    // 如果 T: Send，那么将 T 传到另一个线程中时（按值传送），不会导致数据竞争或其它不安全情况。
    // Send 是对象可以安全发送到另一个执行体中；
    // Send 使被发送对象可以和产生它的线程解耦，防止原线程将此资源释放后，在目标线程中使用出错（use after free）。

    // 如果 T: Sync，那么将 &T 传到另一个线程中时，不会导致数据竞争或其它不安全情况。
    // Sync 是可以被同时多个执行体访问而不出错；
    // Sync 防止的是竞争；

    // 推论：
    // T: Sync 意味着 &T: Send；
    // Sync + Copy = Send；
    // 当 T: Send 时，可推导出 &mut T: Send；
    // 当 T: Sync 时，可推导出 &mut T: Sync；
    // 当 &mut T: Send 时，不能推导出 T: Send；
    // （注：T, &T, &mut T，Box<T> 等都是不同的类型）

    // 具体的类型：

    // 原始类型（比如： u8, f64），都是 Sync，都是 Copy，因此都是 Send；
    // 只包含原始类型的复合类型，都是 Sync，都是 Copy，因此都是 Send；
    // 当 T: Sync，Box<T>, Vec<T> 等集合类型是 Sync；
    // 具有内部可变性的的指针，不是 Sync 的，比如 Cell, RefCell, UnsafeCell；
    // Rc 不是 Sync。因为只要一做 &Rc<T> 操作，就会克隆一个新引用，它会以非原子性的方式修改引用计数，所以是不安全的；
    // 被 Mutex 和 RWLock 锁住的类型 T: Send，是 Sync 的；
    // 原始指针（*mut, *const）既不是 Send 也不是 Sync；

    // Rust 正是通过这两大武器：所有权和生命周期 + Send 和 Sync（本质上为类型系统）来为并发编程提供了安全可靠的基础设施。
    // 使得程序员可以放心在其上构建稳健的并发模型。这也正是 Rust 的核心设计观的体现：内核只提供最基础的原语，真正的实现能
    // 分离出去就分离出去。并发也是如此。
  }

  Ok(())
}