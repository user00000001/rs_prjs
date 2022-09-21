#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // Unsafe / Raw Pointer
  {
    // unsafe
    //
    // 1, deref raw pointer(*mut T / *const T)
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);

    // 2, read/write a mutable static variable
    static mut N: i32 = 5;
    unsafe {
      N += 1;
      println!("N: {}", N);
    }
    // 3, invoke an unsafe function
    unsafe fn foo() {}
    unsafe { foo() };

    // 4, unsafe fn defination
    unsafe fn danger_will_robinson() {}

    // 5, unsafe block
    unsafe {}

    // 6, unsafe trait
    unsafe trait Scary{}
    unsafe impl Scary for i32 {}

    // 7, safe != no bug
    // 不属于“内存安全”的范畴：
    // 死锁
    // 内存或其他资源溢出
    // 退出未调用析构函数
    // 整型溢出

    // 使用unsafe时需要注意一些特殊情形：
    // 数据竞争
    // 解引用空裸指针和悬垂裸指针
    // 读取未初始化的内存
    // 使用裸指针打破指针重叠规则
    // &mut T和&T遵循LLVM范围的noalias模型，除了如果&T包含一个UnsafeCell<U>的话。不安全代码必须不能违反这些重叠（aliasing）保证
    // 不使用UnsafeCell<U>改变一个不可变值/引用
    // 通过编译器固有功能调用未定义行为：
    //    使用std::ptr::offset（offset功能）来索引超过对象边界的值，除了允许的末位超出一个字节
    //    在重叠（overlapping）缓冲区上使用std::ptr::copy_nonoverlapping_memory（memcpy32/memcpy64功能）
    // 原生类型的无效值，即使是在私有字段/本地变量中：
    //    空/悬垂引用或装箱
    //    bool中一个不是false（0）或true（1）的值
    //    enum中一个并不包含在类型定义中判别式
    //    char中一个代理字（surrogate）或超过char::MAX的值
    //    str中非UTF-8字节序列
    // 在外部代码中使用Rust或在Rust中使用外部语言
  }

  {
    // raw pointer: *mut T / *const T
    //
    // can share data with Rc<T> / Arc<T>

    // 不能保证指向有效的内存，甚至不能保证是非空的
    // 没有任何自动清除，所以需要手动管理资源
    // 是普通旧式类型，也就是说，它不移动所有权，因此Rust编译器不能保证不出像释放后使用这种bug
    // 缺少任何形式的生命周期，不像&，因此编译器不能判断出悬垂指针
    // 除了不允许直接通过*const T改变外，没有别名或可变性的保障

    let a = 1;
    let b = &a as *const i32;

    let mut x = 2;
    let y = &mut x as *mut i32;

    let a = 1;
    let b = &a as *const i32;
    let c = unsafe { *b };
    println!("{}", c);

    let a: Box<i32> = Box::new(10);
    let b: *const i32 = &*a as *const i32;
    let c: *const i32 = Box::into_raw(a);
    unsafe {
      println!("{}", *c);
    }
  }

  Ok(())
}